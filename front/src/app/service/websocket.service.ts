import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class WebsocketService {

  private url = 'ws://localhost:8080/person/ws';
  private socket: WebSocket;

  constructor() {
   }

   connect(){
     this.socket = new WebSocket(this.url);
   }

   emit(data?){
     this.socket.send(JSON.stringify(data));
   }

   on(){
     let observable = new Observable<any>(observer => {
       this.socket.onmessage = ((res: any) => {
         observer.next(res);
       });

       return () => this.socket.close.bind(this.socket);
     });
     return observable;
   }
}
