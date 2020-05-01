import {
    Component,
    Input,
    OnChanges,
    SimpleChanges,
    Output,
    EventEmitter,
} from '@angular/core';
import * as Highcharts from 'highcharts';

import { PipsService } from '../pips.service';

export interface ExpressionResult {
    expression: string;
    values: Record<number, number>;
}

@Component({
    selector: 'app-chart',
    templateUrl: './chart.component.html',
    styleUrls: ['./chart.component.scss'],
})
export class ChartComponent implements OnChanges {
    @Input() expressions: string[];
    @Output() expressionsChange = new EventEmitter<string[]>();

    Highcharts = Highcharts;
    options: Highcharts.Options;

    errors: string[] = [];

    constructor(private _pipsService: PipsService) {}

    async ngOnChanges(changes: SimpleChanges) {
        if (changes['expressions']) {
            this.updateChart();
        }
    }

    async plotExpression(expression: string): Promise<ExpressionResult> {
        const result = await this._pipsService.plot(expression);

        if (result.type !== 'Ok') {
            this.errors = [...this.errors, result.value];
            throw new Error(result.value);
        }

        return {
            values: result.value,
            expression,
        };
    }

    removeError(index: number) {
        this.errors.splice(index, 1);
        this.errors = this.errors.slice();
    }

    async updateChart() {
        const results$ = this.expressions.map(async (expr) => {
            try {
                return await this.plotExpression(expr);
            } catch (err) {
                this.expressionsChange.emit(
                    this.expressions.filter((ex) => ex !== expr),
                );
                console.error(err);
                throw err;
            }
        });

        let results: ExpressionResult[];
        try {
            results = await Promise.all(results$);
        } catch (err) {
            return;
        }

        const xValues = results
            .map((result) => Object.keys(result.values))
            .reduce((all, some) => all.concat(some), [])
            .map(Number)
            .filter((x) => !isNaN(x));

        const xMin = Math.min(...xValues);
        const xMax = Math.max(...xValues);

        const series = results.map(
            (result): Highcharts.SeriesLineOptions => {
                const total = Object.values(result.values).reduce(
                    (sum, x) => sum + x,
                    0,
                );
                return {
                    type: 'line',
                    name: result.expression,
                    data: Object.keys(result.values)
                        .map(Number)
                        .filter((x) => !isNaN(x))
                        .sort((a, b) => a - b)
                        .map((x) => [x, (result.values[x] / total) * 100]),
                };
            },
        );

        this.options = {
            title: { text: '' },
            series: series,
            xAxis: {
                allowDecimals: false,
                min: xMin,
                max: xMax,
                title: { text: 'Outcome' },
            },
            yAxis: {
                title: { text: 'Likelihood (%)' },
                labels: {
                    formatter: function () {
                        return `${this.value}%`;
                    },
                },
            },
        };
    }
}
