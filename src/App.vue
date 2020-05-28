<template>
    <div class="container">
        <div class="container__game">
            <div class="header">
                Sudoku
            </div>
            <div class="gridWrapper">
                <div class="grid">
                    <div v-bind:class="{ 'item__row': item.row % 3 === 0, 'item__column': item.column % 3 === 0}"
                        class="item" :key="item.id" v-for="item of state.grid">
                        <input 
                            ref="cellRefs"
                            :class="computeClass(item)"
                            class="input" type="text"
                            @mouseenter="onMouseEnter(item)"
                            @mouseleave="onMouseLeave(item)"
                            @focus="onFocus(item)"
                            @keydown="onKeyDown(item, $event)"
                            :disabled="item.disabled"
                            :value="item.value" @input="onInput($event, item)">
                    </div>
                </div>
            </div>
            <div class="actions">
                <div class="action" @click="reset()">
                    <v-icon class="icon" :large="true" color="red">{{ icons.mdiRefresh }}</v-icon>
                </div>
            </div>
        </div>
        <div class="footer">
            <div class="footer__author">{{state.signature}}</div>
            <div class="footer__social">
                <a class="footer__link" :key="item.id" :href="item.href" v-for="item of state.social">
                    <i :class="item.className"></i>
                <a>
            </div>
        </div>
    </div>
</template>

<script lang="ts">
 import {
    mdiRefresh
  } from '@mdi/js'

import { reactive, computed, ref, watch, onMounted } from '@vue/composition-api'
import { generateFilledGrid, getProfileInfo, numberOfrows, numberOfcolumns, cursorMove, getNextCell } from './api';
import { Cell } from './models/Cell';
import { Social } from './models/Social';

export default {
    setup() {
        const numberOfFilledCells = 20;

        const state = reactive<{
            grid: Cell[],
            social: Social[],
            signature: string;
        }>({
            grid: [],
            social: [],
            signature: '',
        });

        onMounted(() => {
            getProfileInfo().then(data => {
                state.signature = data.signature;
                state.social = data.social;
            })

            generateFilledGrid(numberOfrows, numberOfcolumns, numberOfFilledCells)
                .then(data => {
                    state.grid = data.grid;
                })
        })
   
        const cellRefs = ref<HTMLInputElement[]>(null);


        const fill = (list: any[], index: number, value: number) => {
            let container = list[index];

            if(container.values.has(value)) {
                container.valid = false;
            }
            
            container.values.add(value);
        }

        const onFocus = (item: any) => {
            state.grid = state.grid.map((cell: any) => ({
                ...cell,
                active: item.block === cell.block
            }))
        }

        const reset = () => {
            generateFilledGrid(numberOfrows, numberOfcolumns, numberOfFilledCells)
                .then(data => {
                    
                    state.grid = data.grid;
                })
        }

        const computeClass = (item: any) => {
            if(item.disabled) {
                return {
                    'input--disabled': true
                }
            }

            if(item.invalid) {
                return {
                    'input--invalid': true
                }
            }
            
            if(item.active) {
                return {
                    'input--active': true
                }
            }

            if(item.hover) {
                return {
                    'input--hover': true
                }
            }
            
        }

        const onMouseEnter = (item: any) => {
            state.grid = state.grid.map((cell: any) => ({
                ...cell,
                hover: item.block === cell.block ? true : false
            }))
        }

        const onMouseLeave = (item: any) => {
            state.grid = state.grid.map((cell: any) => ({
                ...cell,
                hover: false
            }))
        }

        const onInput = (event: any, item: any) => {
            const value = event.target.value;
            item.value = value ? Number(value) : null;
            validate(item.id);
        }

        const validate = (id: number) => {
            
            const result = state.grid.reduce((acc: any, cell: any) => {
                
                if(cell.value) {
                    fill(acc.blocks, cell.block, cell.value);
                }

                if(cell.value) {
                    fill(acc.rows, cell.row, cell.value);
                }

                if(cell.value) {
                    fill(acc.columns, cell.column, cell.value);
                }
                
                return acc;
            }, {
                blocks: Array(9).fill(0).map(pr => ({
                    values: new Set<number>(),
                    valid: true
                })),
                rows: Array(9).fill(0).map(pr => ({
                    values: new Set<number>(),
                    valid: true
                })),
                columns: Array(9).fill(0).map(pr => ({
                    values: new Set<number>(),
                    valid: true
                })),
            });

            state.grid = state.grid.map((cell: any) => {
                let invalid = false;

                if(!result.blocks[cell.block].valid) {
                    invalid = true;
                }

                if(!result.rows[cell.row].valid) {
                    invalid = true;
                }

                if(!result.columns[cell.column].valid) {
                    invalid = true;
                }

                return {
                    ...cell,
                    invalid
                }
            });            
        }

        const onKeyDown = (item: any, event: KeyboardEvent & {target: HTMLInputElement}) => {
            const key = event.key;
            const regex = /[1-9]|\./;
            const value = event.target.value;
            const containsValue = value.length !== 0;
            
            const direction = cursorMove(key);

            if(direction && cellRefs.value) {

                const id = getNextCell(state.grid, item, direction);

                if(!id) {
                    return;
                }

                const element = cellRefs.value[id];

                element.focus();

                return event.preventDefault();
            }
 
            if(key !== "Backspace" && (containsValue || !regex.test(key))) {
                return event.preventDefault();
            }
        }


        return {
            state,
            validate,
            onFocus,
            onInput,
            onKeyDown,
            onMouseEnter,
            onMouseLeave,
            cellRefs,
            reset,
            computeClass,
            icons: {
                mdiRefresh
            }
        };
    }
};
</script>

<style lang="scss" scoped>

    .container {
        height: 100%;
        display: flex;
        flex-direction: column;

        &__game {
            flex: 1 1 auto;
        }
    }

    .header {
        font-size: 4rem;
        text-align: center;
        padding: 3rem;
    }

    .gridWrapper {
        text-align: center;
        display: flex;
        justify-content: center;
    }

    .grid {
        display: grid;
        grid-template-columns: repeat(9, 50px);
        grid-template-rows: repeat(auto-fit, 50px);
        height: 100%;
    }
    
    .item {
        border: .5px solid black;

        &__row {
            border-top: 2px solid black;
        }

        &__column {
            border-left: 2px solid black;
        }
    }

    .input {
        width: 100%;
        height: 100%;
        padding: .5rem;
        font-size: 2rem;
        text-align: center;
        border: 1px solid black;
        border: none;
        background: lightgray;
        transition: background .3s ease-in, color .3s ease-in;

        &:focus {
            outline: none;
        }

        &--disabled {
            background: gray;
            color: white;
        }

        &--disabled {
            background: transparentize(lightgray, 0.5);
        }

        &--active {
            background: #FFFFFF;
            color: black;
        }

        &--hover {
            background: #FFFFFFEE;
            color: black;
        }

        &--invalid {
            background: #990F04;
            color: white;
        }
    }

    .icon {
        fill: white;
        cursor: pointer;

    }

    .actions {
        text-align: center;
        margin-top: 2rem;
    }

    .footer {
        display: flex;
        justify-content: space-between;
        padding: 2rem;
        flex: 0 0 auto;

        &__author {
            font-size: 1.5rem;
        }

        &__link {
            color: white;
            margin: 0 10px;
        }
    }

</style>