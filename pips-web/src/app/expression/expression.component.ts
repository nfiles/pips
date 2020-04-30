import {
    Component,
    Output,
    EventEmitter,
    OnInit,
    ElementRef,
    OnDestroy,
} from '@angular/core';
import { FormGroup, FormControl } from '@angular/forms';
import { fromEvent, Subject } from 'rxjs';
import { takeUntil, filter } from 'rxjs/operators';

import { PipsService } from '../pips.service';

@Component({
    selector: 'app-expression',
    templateUrl: './expression.component.html',
    styleUrls: ['./expression.component.scss'],
})
export class ExpressionComponent implements OnInit, OnDestroy {
    @Output() onPlot = new EventEmitter<string>();

    form = new FormGroup({
        expression: new FormControl(''),
    });

    result: string;

    private _isDestroyed = new Subject<void>();

    constructor(
        private _pipsService: PipsService,
        private _element: ElementRef,
    ) {}

    ngOnInit() {
        fromEvent<KeyboardEvent>(this._element.nativeElement, 'keydown')
            .pipe(
                takeUntil(this._isDestroyed),
                filter((evt) => evt.ctrlKey),
                filter((evt) => evt.keyCode === 13 || evt.keyCode === 10),
                filter((_) => this.form.valid),
            )
            .subscribe((_) => {
                this.onPlot.emit(this.form.value.expression);
            });

        this.form.valueChanges
            .pipe(takeUntil(this._isDestroyed))
            .subscribe((_) => {
                this.onPlot.emit(null);
                this.result = '';
            });
    }

    ngOnDestroy() {
        this._isDestroyed.next();
        this._isDestroyed.complete();
    }

    async roll(expression: string) {
        this.result = '...';
        const result = await this._pipsService.roll(expression);
        this.result = String(result.result);
    }
}
