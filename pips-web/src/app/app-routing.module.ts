import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { VisualizerComponent } from './visualizer/visualizer.component';
import { RouteData } from './route-data.model';

const routes: Routes = [
    {
        component: VisualizerComponent,
        path: 'visualizer',
        data: { title: 'Visualizer' } as RouteData,
    },
    {
        path: '',
        pathMatch: 'full',
        redirectTo: 'visualizer',
    },
];

@NgModule({
    imports: [RouterModule.forRoot(routes)],
    exports: [RouterModule],
})
export class AppRoutingModule {}
