import { Component, Input, OnChanges, SimpleChanges } from '@angular/core';
import * as Highcharts from 'highcharts';

import { PipsService } from '../pips.service';

export interface Expression {
    expression: string;
    values: Record<number, number>;
}

@Component({
    selector: 'app-chart',
    templateUrl: './chart.component.html',
    styleUrls: ['./chart.component.scss'],
})
export class ChartComponent implements OnChanges {
    @Input() expression: string;

    Highcharts = Highcharts;
    options: Highcharts.Options;

    expressions: Expression[] = [];

    xMin = 0;
    xMax = 0;

    error: string;

    constructor(private _pipsService: PipsService) {}

    async ngOnChanges(changes: SimpleChanges) {
        if (changes['expression']) {
            this.error = null;

            if (this.expression) {
                this.plotExpression(this.expression);
            }
        }
    }

    async plotExpression(expression: string) {
        // skip if it's already plotted
        if (this.expressions.some((ex) => expression === ex.expression)) {
            return;
        }

        const result = await this._pipsService.plot(expression);

        if (result.type !== 'Ok') {
            this.error = result.value;
            return;
        }

        this.addSeries({
            values: result.value,
            expression,
        });
    }

    addSeries(expression: Expression) {
        const newXValues = Object.keys(expression.values)
            .map(Number)
            .filter((x) => !isNaN(x));

        this.xMin = Math.min(...newXValues, this.xMin);
        this.xMax = Math.max(...newXValues, this.xMax);

        this.expressions = [...this.expressions, expression];

        this.updateChart();
    }

    removeSeries(expression: Expression) {
        this.expressions = this.expressions.filter((ex) => ex !== expression);

        const xValues = this.expressions
            .map((expr) => Object.keys(expr.values))
            .reduce((all, some) => all.concat(some), [])
            .map(Number)
            .filter((x) => !isNaN(x));

        this.xMin = Math.min(...xValues);
        this.xMax = Math.max(...xValues);

        this.updateChart();
    }

    clearSeries() {
        this.xMin = 0;
        this.xMax = 0;
        this.expressions = [];

        this.updateChart();
    }

    updateChart() {
        const series = this.expressions.map(
            (expr): Highcharts.SeriesLineOptions => {
                return {
                    type: 'line',
                    name: expr.expression,
                    data: Object.keys(expr.values)
                        .map(Number)
                        .filter((x) => !isNaN(x))
                        .sort((a, b) => a - b)
                        .map((x) => [x, expr.values[x]]),
                };
            },
        );

        this.options = {
            title: { text: '' },
            series: series,
            xAxis: {
                allowDecimals: false,
                min: this.xMin,
                max: this.xMax,
            },
        };
    }
}
