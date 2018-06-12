import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { Person } from '../model/person';

class Response{
  status: number
  message: string
  person_list: Person[]
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
}
