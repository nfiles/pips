import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { AnalyzerComponent } from './analyzer/analyzer.component';

const routes: Routes = [
    { component: AnalyzerComponent, path: 'analyzer' },
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
