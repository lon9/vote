import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { AppConfig } from '../config/app.config';

@Injectable({
  providedIn: 'root'
})
export class WebsocketService {

  private url = AppConfig.endpoints.ws;
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
