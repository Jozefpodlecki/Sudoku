import { Cell } from "./models/Cell";
import { Profile } from "./models/Profile";

const randomValue = () => Math.floor(Math.random() * 9) + 1;
const randomId = () => Math.floor(Math.random() * 81);

export const getProfileInfo = () => new Promise<Profile>((resolve, reject) => {
    const currentYear = new Date().getFullYear();
        const signature = `Józef Podlecki ${currentYear}`;
        const social = [
            {
                id: 1,
                href: 'https://github.com/Jozefpodlecki',
                className: 'fab fa-github fa-2x'
            },
            {
                id: 2,
                href: 'https://linkedin.com/in/jozef-witold-podlecki',
                className: 'fab fa-linkedin fa-2x'
            },
            {
                id: 3,
                href: 'https://www.freecodecamp.org/jozefpodlecki',
                className: 'fab fa-free-code-camp fa-2x'
            }
        ];

    resolve({
        signature,
        social
    })
})

export const createGrid = (numberOfrows: number, numberOfcolumns: number) => {
    const data: Cell[] = Array(numberOfrows * numberOfcolumns).fill(0).map((pr, index) => {
        return {
            id: index,
            value: null as null | number,
            row: Math.floor(index / numberOfrows),
            column: index % numberOfcolumns,
            block: Math.floor((index % 9) / 3) + 3 * Math.floor(index / 27),
            invalid: false,
            disabled: false,
            hover: false,
            active: false
        }
    })

    return data;
}

const _fill = () => Array(9).fill(0).map(pr => ({
    values: new Set<number>(),
    valid: true
}));

type Container = {
    values: Set<number>;
    valid: boolean;
}

type State = {
    grid: Cell[],
    blocks: Container[],
    rows: Container[],
    columns: Container[]
}

export const generateFilledGrid = (numberOfrows: number, numberOfcolumns: number, numberOfFilledCells: number = 10) => new Promise<State>((resolve, reject) => {
    const grid = createGrid(numberOfrows, numberOfcolumns);
    let ids = new Set(); 
    const blocks = _fill();
    const rows = _fill();
    const columns = _fill();

    for(let _ of Array(numberOfFilledCells)) {
        let id = randomId();

        while(ids.has(id)) {
            id = randomId();
        }

        ids.add(id);

        const cell = grid[id];
        const { block, row, column } = cell;
        
        let value = randomValue();
        const blockValues = blocks[block].values;
        const rowValues = rows[row].values;
        const columnValues = columns[column].values;

        while(blockValues.has(value) ||
        rowValues.has(value) ||
        columnValues.has(value)) {
            value = randomValue();
        }

        blockValues.add(value);
        rowValues.add(value);
        columnValues.add(value);

        cell.value = value;
        cell.disabled = true;
    }

    resolve({
        grid,
        blocks,
        rows,
        columns
    })
})

const getByRowAndColumn = (grid: Cell[], row: number, column: number) => {
    const relativeItem = grid.find(pr => pr.row === row && pr.column === column);

    if(!relativeItem) {
        return null;
    }

    return relativeItem;
}

export const numberOfrows = 9;
export const numberOfcolumns = 9;

type DirectionAction = (row: number, column: number) => {
    row: number, column: number
}

type DirectionActionDict = {
    [index: string]: DirectionAction | null
}

const map: DirectionActionDict = {
    ArrowUp: (row: number, column: number) => {
        row = row - 1;

        if(row === -1) {
            row = numberOfrows - 1;
        }
        
        return {
            row,
            column
        }
    },
    ArrowLeft: (row: number, column: number) => {
        column = column - 1;

        if(column === -1) {
            column = numberOfcolumns - 1;
        }

        return {
            row,
            column
        }
    },
    ArrowRight: (row: number, column: number) => {
        column = column + 1;

        if(column === numberOfcolumns) {
            column = 0;
        }

        return {
            row,
            column
        }
    }, 
    ArrowDown: (row: number, column: number) => {
        row = row + 1;

        if(row === numberOfrows) {
            row = 0;
        }

        return {
            row,
            column
        }
    }
};


export const cursorMove = (key: string) => {
    const directionAction = map[key];

    return directionAction;
}

export const getNextCell = (grid: Cell[], item: Cell, nextDirection: DirectionAction) => {

    let direction = nextDirection(item.row, item.column);
    let element = getByRowAndColumn(grid, direction.row, direction.column);

    while(element && element.disabled) {
        direction = nextDirection(direction.row, direction.column);
        element = getByRowAndColumn(grid, direction.row, direction.column);
    }

    return element && element.id;
}