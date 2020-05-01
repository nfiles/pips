import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { NgbModule } from '@ng-bootstrap/ng-bootstrap';
import { ReactiveFormsModule } from '@angular/forms';
import { HighchartsChartModule } from 'highcharts-angular';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { ChartComponent } from './chart/chart.component';
import { ExpressionComponent } from './expression/expression.component';
import { AnalyzerComponent } from './analyzer/analyzer.component';

@NgModule({
    declarations: [
        AppComponent,
        ChartComponent,
        ExpressionComponent,
        AnalyzerComponent,
    ],
    imports: [
        BrowserModule,
        AppRoutingModule,
        NgbModule,
        ReactiveFormsModule,
        HighchartsChartModule,
    ],
    providers: [],
    bootstrap: [AppComponent],
})
export class AppModule {}
