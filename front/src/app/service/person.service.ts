import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { Person } from '../model/person';

class Response{
  status: number
  message: string
  person_list: Person[]
}

export class Op{

  code: string;
  person: Person;

  constructor(
    code: string,
    person: Person
  ) { 
    this.code = code;
    this.person = person;
  }
}

export class VoteInfo{

  ops: Op[];
  created: Date;

  constructor(){
    this.created = new Date();
    this.ops = [];
  }
}

@Injectable({
  providedIn: 'root'
})
export class PersonService {

  constructor(
    private http: HttpClient
  ) { }

  getPersons(): Observable<Response>{
    return this.http.get<Response>('http://localhost:8080/api/person/list');
  }

  getVoteInfo(): VoteInfo {
    let voteInfo = JSON.parse(localStorage.getItem('voteInfo'));
    if(voteInfo == undefined) {
      voteInfo = new VoteInfo();
    }
    return voteInfo;
  }

  setVoteInfo(info: VoteInfo) {
    localStorage.setItem('voteInfo', JSON.stringify(info));
  }

  removeVoteInfo(){
    localStorage.removeItem('voteInfo');
  }
}
