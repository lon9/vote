import { Component, OnInit } from '@angular/core';
import { Person } from '../model/person';
import { PersonService } from '../service/person.service';
import { WebsocketService } from '../service/websocket.service';

@Component({
  selector: 'app-person',
  templateUrl: './person.component.html',
  styleUrls: ['./person.component.css']
})
export class PersonComponent implements OnInit {

  persons: Person[];
  private connection: any;
  displayedColumns = ['position', 'name', 'vote', 'upvote', 'downvote']

  constructor(
    private personService: PersonService,
    private websocketService: WebsocketService
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

  }

  sortPersons(): void{
    this.persons.sort((a,b) => {
      if(a.vote > b.vote) return -1;
      if(a.vote < b.vote) return 1;
      return 0;
    });
  }

  upvote(person: Person): void{
    this.websocketService.emit({
      person_id: person.id,
      op: '+'
    });
  }

  downvote(person: Person): void{
    this.websocketService.emit({
      person_id: person.id,
      op: '-'
    });
  }

}
