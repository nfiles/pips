import { Injectable } from '@angular/core';

const pips$ = import('pips-wasm');

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
