/**
 * Round a number to a number of decimal places
 * @param num a number
 * @param decimals a number of decimal places
 */
export function Round(num: number, decimals: number): number {
    const factor = Math.pow(10, decimals);
    return Math.round(num * factor) / factor;
}
