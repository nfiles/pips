import { Component, Input, OnChanges, SimpleChanges } from '@angular/core';
import { PipsService } from '../pips.service';
import { Result } from 'pips-wasm';

@Component({
    selector: 'app-chart',
    templateUrl: './chart.component.html',
    styleUrls: ['./chart.component.scss'],
})
export class ChartComponent implements OnChanges {
    @Input() expression: string;

    result: Result<Record<number, number>, string>;

    constructor(private _pipsService: PipsService) {}

    async ngOnChanges(changes: SimpleChanges) {
        if (changes['expression']) {
            this.result = null;
            if (this.expression) {
                this.result = await this._pipsService.plot(this.expression);
            }
        }
    }
}
