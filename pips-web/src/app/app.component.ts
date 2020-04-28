import { Component } from '@angular/core';
import('pips-wasm')
  .then(pips => {
    (window as any)['roll'] = (input: string) => pips.roll(input);
    (window as any)['plot'] = (input: string) => pips.plot(input);
  })
  .catch(err => {
    console.error('error importing pips-wasm', err);
  });

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title = 'pips-ng';
}
