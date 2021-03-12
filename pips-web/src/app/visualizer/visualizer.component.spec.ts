import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { VisualizerComponent } from './visualizer.component';

describe('AnalyzerComponent', () => {
    let component: VisualizerComponent;
    let fixture: ComponentFixture<VisualizerComponent>;

    beforeEach(async(() => {
        TestBed.configureTestingModule({
            declarations: [VisualizerComponent],
        }).compileComponents();
    }));

    beforeEach(() => {
        fixture = TestBed.createComponent(VisualizerComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it('should create', () => {
        expect(component).toBeTruthy();
    });
});