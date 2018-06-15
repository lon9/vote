import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Routes, RouterModule } from '@angular/router';
import { MatButtonModule, MatTableModule, MatSnackBarModule, MatListModule, MatExpansionModule, MatCardModule } from '@angular/material';

import { PersonComponent } from './person.component';
import { FlexLayoutModule } from '@angular/flex-layout';
import { AppConfig } from '../config/app.config';

const routes: Routes = [
  {
    path: AppConfig.routes.person,
    component: PersonComponent
  }
]

@NgModule({
  imports: [
    CommonModule,
    RouterModule.forChild(routes),
    MatButtonModule,
    MatListModule,
    MatTableModule,
    MatSnackBarModule,
    MatExpansionModule,
    MatCardModule,
    FlexLayoutModule
  ],
  exports: [
    RouterModule
  ],
  declarations: [
    PersonComponent
  ]
})
export class PersonModule { }
