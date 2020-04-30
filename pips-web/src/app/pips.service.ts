import { Injectable } from '@angular/core';

const pips$ = import('pips-wasm')
    .then((pips) => {
        (window as any)['roll'] = (input: string) => pips.roll(input);
        (window as any)['plot'] = (input: string) => pips.plot(input);
        return pips;
    })
    .catch((err) => {
        console.error('error importing pips-wasm', err);
        throw err;
    });

@Injectable({
    providedIn: 'root',
})
export class PipsService {
    async roll(input: string) {
        const pips = await pips$;
        return pips.roll(input);
    }

    async plot(input: string) {
        const pips = await pips$;
        return pips.plot(input);
    }
}
