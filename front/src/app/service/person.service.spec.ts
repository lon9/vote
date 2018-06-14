import { TestBed, inject } from '@angular/core/testing';

import { PersonService } from './person.service';
import { HttpClientModule } from '@angular/common/http';

describe('PersonService', () => {
  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [
        HttpClientModule
      ],
      providers: [PersonService]
    });
  });

  it('should be created', inject([PersonService], (service: PersonService) => {
    expect(service).toBeTruthy();
  }));
});
