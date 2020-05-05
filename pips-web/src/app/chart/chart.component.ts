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
import { Round } from '../../utilities/numbers';

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

        const allData = results.map((result) => {
            const total = Object.values(result.values).reduce(
                (sum, x) => sum + x,
                0,
            );
            return {
                result,
                points: Object.keys(result.values)
                    .map(Number)
                    .filter((x) => !isNaN(x))
                    .sort((a, b) => a - b)
                    .map<[number, number]>((x) => [
                        x,
                        // normalize the y value
                        result.values[x] / total,
                    ]),
                total: total,
            };
        });

        const series = allData.map(
            ({ result, points }): Highcharts.SeriesOptionsType => {
                return {
                    type: 'line',
                    name: result.expression,
                    data: points.map(([x, y]) => [x, Round(y * 100, 2)]),
                };
            },
        );

        const meanLines = allData.map(
            ({ result, points }): Highcharts.XAxisPlotLinesOptions => {
                const average = points.reduce((sum, [x, y]) => sum + x * y, 0);

                return {
                    dashStyle: 'Dash',
                    value: average,
                    label: {
                        text: `${result.expression}<br>${Round(average, 2)}`,
                        rotation: 0,
                    },
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
                plotLines: meanLines,
            },
            yAxis: {
                title: { text: '' },
                labels: {
                    formatter() {
                        return `${this.value}%`;
                    },
                },
            },
        };
    }
}
