import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { AnalyzerComponent } from './analyzer/analyzer.component';
import { RouteData } from './route-data.model';

const routes: Routes = [
    {
        component: AnalyzerComponent,
        path: 'analyzer',
        data: { title: 'Analyzer' } as RouteData,
    },
    {
        path: '',
        pathMatch: 'full',
        redirectTo: 'analyzer',
    },
];

@NgModule({
    imports: [RouterModule.forRoot(routes)],
    exports: [RouterModule],
})
export class AppRoutingModule {}
