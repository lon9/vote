import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Routes, RouterModule } from '@angular/router';
import { MatListModule, MatIconModule, MatButton, MatButtonModule } from '@angular/material';

import { PersonComponent } from './person.component';

const routes: Routes = [
  {
    path: '',
    component: PersonComponent
  }
]

@NgModule({
  imports: [
    CommonModule,
    RouterModule.forChild(routes),
    MatListModule,
    MatIconModule,
    MatButtonModule
  ],
  exports: [
    RouterModule
  ],
  declarations: [
    PersonComponent
  ]
})
export class PersonModule { }
