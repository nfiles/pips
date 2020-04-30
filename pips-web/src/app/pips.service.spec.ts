import { TestBed } from '@angular/core/testing';

import { PipsService } from './pips.service';

describe('PipsService', () => {
    let service: PipsService;

    beforeEach(() => {
        TestBed.configureTestingModule({});
        service = TestBed.inject(PipsService);
    });

    it('should be created', () => {
        expect(service).toBeTruthy();
    });
});
