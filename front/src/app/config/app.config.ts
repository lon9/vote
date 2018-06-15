import { IAppConfig } from "./iapp.config";
import { InjectionToken } from "@angular/core";

export let APP_CONFIG = new InjectionToken('app.config');

export const AppConfig: IAppConfig = {
    routes: {
        person: ''
    },
    endpoints: {
        personList: 'http://localhost:8080/api/person/list',
        ws: 'ws://localhost:8080/person/ws'
    },
    votesLimit: 5,
    snackBarDuration: 810
}