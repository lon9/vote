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

  private persons: Person[];
  private connection;

  constructor(
    private personService: PersonService,
    private websocketService: WebsocketService
  ) { }

  ngOnInit() {
    this.personService.getPersons()
      .subscribe(res => {
        this.persons = res.person_list;
        this.websocketService.connect();
        this.connection = this.websocketService.on()
          .subscribe(
            (res) => {
              console.log(res.data);
              this.persons.filter((person) => {person.id == res.data.person_id}).map((person, idx) => {
                if(res.data.op == '+') person[idx].vote++;
                else if(res.data.op == '-') person[idx].vote--;
              });
              this.persons = this.persons.map((person) => {
                if(person.id == res.data.person_id){
                  
                }
              });
            }
          );
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
