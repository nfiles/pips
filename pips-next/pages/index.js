import Link from 'next/link';
import('pips-wasm').then(
    pips => {
        window['roll'] = pips.roll;
        window['plot'] = pips.plot;
    },
    err => console.error('pips import error', err),
);

const Index = () => (
    <div>
        <Link href="/about">
            <button>Go to About Page</button>
        </Link>
        <p>Hello Next.js</p>
    </div>
);

export default Index;
