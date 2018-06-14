import { Component, OnInit } from '@angular/core';
import { Person } from '../model/person';
import { PersonService, VoteInfo, Op } from '../service/person.service';
import { WebsocketService } from '../service/websocket.service';
import { MatSnackBar } from '@angular/material';

@Component({
  selector: 'app-person',
  templateUrl: './person.component.html',
  styleUrls: ['./person.component.css']
})
export class PersonComponent implements OnInit {

  persons: Person[];
  private connection: any;
  displayedColumns = ['position', 'name', 'vote', 'upvote', 'downvote'];
  voteInfo: VoteInfo;

  constructor(
    private personService: PersonService,
    private websocketService: WebsocketService,
    public snackBar: MatSnackBar
  ) { }

  ngOnInit() {
    this.personService.getPersons()
      .subscribe(res => {
        this.persons = res.person_list;
        this.sortPersons();
        this.websocketService.connect();
        this.connection = this.websocketService.on()
          .subscribe(
            (res) => {
              const data = JSON.parse(res.data);
              this.persons = this.persons.map((person) => {
                if(person.id == data.person_id){
                  if(data.op == '+'){
                    person.vote++;
                    return person;
                  }else if(data.op == '-'){
                    person.vote--;
                    return person;
                  }
                }
                return person;
              });
              this.sortPersons();
            }
          );
      });
      this.refreshInfo();
  }

  sortPersons(): void{
    this.persons.sort((a,b) => {
      if(a.vote > b.vote) return -1;
      if(a.vote < b.vote) return 1;
      return 0;
    });
  }

  refreshInfo(): void{
    this.voteInfo = this.personService.getVoteInfo();
    const now = new Date();
    const created = new Date(this.voteInfo.created);
    if(now.getDate() - created.getDate() >1){
      this.personService.removeVoteInfo();
      this.voteInfo = this.personService.getVoteInfo();
    }
  }

  canVote(): boolean{
    if(this.voteInfo.ops.length > 4){
      return false;
    }
    return true;
  }

  vote(person: Person, op: string): void{
    this.refreshInfo();
    if(!this.canVote()){
      this.snackBar.open('1日5回しか投票できません。また明日投票してください。');
      return;
    }
    this.websocketService.emit({
      person_id: person.id,
      op: op
    });
    this.voteInfo.ops.push(new Op(
      op,
      person
    ));
    this.personService.setVoteInfo(this.voteInfo);
  }

}
