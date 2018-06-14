import { async, ComponentFixture, TestBed, fakeAsync, tick } from '@angular/core/testing';

import { PersonComponent } from './person.component';
import { MatTableModule, MatSnackBarModule } from '@angular/material';
import { HttpClientModule } from '@angular/common/http';
import { VoteInfo, PersonService } from '../service/person.service';
import { WebsocketService } from '../service/websocket.service';

describe('PersonComponent', () => {
  let component: PersonComponent;
  let fixture: ComponentFixture<PersonComponent>;

  let webSocketService: WebsocketService;
  let personService: PersonService;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      imports: [
        MatTableModule,
        MatSnackBarModule,
        HttpClientModule
      ],
      declarations: [ PersonComponent ],
      providers: [
        WebsocketService,
        PersonService
      ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    localStorage.setItem('voteInfo', JSON.stringify(new VoteInfo()));
    fixture = TestBed.createComponent(PersonComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
    webSocketService = TestBed.get(WebsocketService);
    personService = TestBed.get(PersonService);
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
