export interface Cell {
    id: number;
    value: null | number;
    row: number;
    column: number;
    block: number;
    invalid: boolean;
    disabled: boolean;
    hover: boolean;
    active: boolean;
}