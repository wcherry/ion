import { saveBlock as saveBlockAPI} from './service.mjs';


export function createIonBlockElement(block) {
    const blockElement = document.createElement('ion-block');
    const {id, blockType, content, writable} = block;
    blockElement.id = id;
    blockElement.setAttribute('type', blockType);
    blockElement.setAttribute('writable', writable);
    blockElement.block = block;
    if(blockType=='table') {
        buildTable(blockElement, content);
    } else {
        blockElement.innerHTML = JSON.parse(content || {})?.content;
    }
    return blockElement;
}

export class BlockElement extends HTMLElement {
    static get observedAttributes() {
        return ['name', 'type', 'menu', 'writable', 'dirty']
    }

    set block(block) {
        this._block = block;
    }

    get block() {
        return this._block;
    }

    static new(block) {
        const blockElement = document.createElement('ion-block');
        const {id, blockType, content, writable} = block;
        blockElement.block = block;
        blockElement.id = id;
        console.log("Creating new block", blockType, block);
        blockElement.setAttribute('type', blockType);
        blockElement.setAttribute('writable', writable || true);
        switch(blockType) {
            case 'table':
                this.buildTable(blockElement, content);
                break;
            default:
                this.buildParagraph(blockElement, content);
        }
        return blockElement;
    }

    static load(block) {
        const blockElement = document.createElement('ion-block');
        const {id, blockType, content, writable, blockId, pageVersionId} = block;
        blockElement.id = id;
        blockElement.setAttribute('type', blockType);
        blockElement.setAttribute('writable', writable);
        blockElement.blockId = blockId;
        blockElement.pageVersionId = pageVersionId;
        blockElement.block = block;
        switch(blockType) {
            case 'table':
                this.loadTable(blockElement, content);
                break;
            default:
                this.loadParagraph(blockElement, content);
        }
        return blockElement;
    }

    isDirty() {
        return this.attributes.dirty?.value;
    }

    async saveBlock() {
        const blockType = this.attributes.type.value || 'paragraph';
        const payload = {...this.block, blockType};
        switch(this.attributes.type.value) {
            case 'table':
                payload.content = this.toJson();
                break;
            default:
                payload.content = JSON.stringify({content: this.querySelector('.editable').textContent});
        }
        const resp = await saveBlockAPI(payload);
        this.setAttribute("dirty", false);
        this.id = resp.blockId;
        this.block = {...this.block, ...resp};
        // console.info("BLOCK SAVED", resp);
    }

/*
        // if(this.isDirty()) {
            if(this.attributes.type.value=='table') {
                let headers = []; 
                this.querySelectorAll('th').forEach(th => {
                    headers.push(th.textContent);    
                });
                payload.headers = headers;
                let data = [];
                this.querySelectorAll('tr').forEach(tr => {
                    let row = [];
                    tr.querySelectorAll('td').forEach(td => {
                        row.push(td.textContent);    
                    });
                    data.push(row);
                });
                payload.data = data;
            } else {
                payload.data = this.querySelector('.editable').textContent;
            }
*/

    startEdit() {
        let p = this.querySelector('.editable')
        p.focus();
        // Move cursor to end of content
        let range = document.createRange();
        range.selectNodeContents(p);
        range.collapse(false);
        let sel = window.getSelection();
        sel.removeAllRanges();
        sel.addRange(range);
    }

    connectedCallback() {
        const self = this;
        this.setAttribute('tabindex', '0');
        const id = this.attributes.id.value;
        const type = this.attributes.type.value;
        const writable = this.attributes.writable ? this.attributes.writable.value : false;
        const body = self.innerHTML;
        // BUG: If the user doesn't have permission to edit, the contenteditable attribute is still set to true
        // TODO: If user doesn't have permission to edit, remove contenteditable="true"
        self.innerHTML = `<div class="element" data-type="${type}"><menu-toggle></menu-toggle><div class="el editable ${type}" contenteditable="${writable}">${body}</div></div>`;   
        
        const editableElement = this.querySelector(".editable");

        editableElement.addEventListener('keypress', (e) => {
            if(e.keyCode === 13) {
                e.preventDefault();
                e.stopPropagation();
                // if(self.isDirty()) {
                //     saveBlock({...this._block, content:editableElement.textContent});
                // }
                const el = BlockElement.new({blockType:'paragraph', content:'', writable:true, pageVersionId: this._block.pageVersionId, displayOrder: this._block.displayOrder+1});
                document.querySelector('.body').append(el);
                self.blur();
                el.focus();
                setTimeout(function() {
                    el.startEdit();
                }, 0);

                return false;
            }
        });

        editableElement.addEventListener('focus', (e) => {      
            //TODO: record the current state of the element
            this.dataset.state = editableElement.textContent;
        });
        editableElement.addEventListener('blur', (e) => {       
            //TODO: check if the element has changed, if so, save it
            if(this.dataset.state != editableElement.textContent) {
                this.setAttribute("dirty", true);
            }
        });
    }
    
    attributeChangedCallback(name, oldValue, newValue) {
        const editableElement = this.querySelector(".editable");
        if(name == 'type' && editableElement) {
            editableElement.classList.remove(oldValue);
            editableElement.classList.add(newValue);
            this.saveBlock();
        }
        if(name == 'menu') {
            const menuToggle = this.querySelector('menu-toggle');
            this.querySelector('.menu-toggle').addEventListener('click', (e) => newValue(e, this));
        }
        if(name == 'writable'  && editableElement) {
            if(newValue == 'true') {
                editableElement.setAttribute('contenteditable', 'true');
            } else {
                editableElement.removeAttribute('contenteditable');
            }
        }
        if(name == 'dirty' && newValue == 'true' && (oldValue == undefined || oldValue == 'false')) {
            this.saveBlock();
        }
    }
    
    setMenu(menuHandler) {
        this.querySelector('.menu-toggle').addEventListener('click', (e) => menuHandler(e, this));
    }



    static buildParagraph(blockElement, content) {
        // blockElement.toJson = function() {
        //     return JSON.stringify(
        //         {type: 'paragraph', 
        //             content: this.querySelector('.editable').textContent,
        //         s});
        // }
        blockElement.id = "unsaved-"+Date.now();
        blockElement.innerHTML = content;
        blockElement.writable = true;
    }

    static loadParagraph(blockElement, content) {
        let data = JSON.parse(content || {});
        blockElement.innerHTML = data.content || '';
    }

    static buildTable(blockElement) {
        // add json method
        blockElement.toJson = function() {
            let data = {type: 'table', headers: [], rows: [], style: this.attributes.style?.value || 'table-striped'};
            this.querySelectorAll('th').forEach(th => {
                data.headers.push(th.textContent);    
            });
            this.querySelectorAll('tr').forEach(tr => {
                let row = [];
                tr.querySelectorAll('td').forEach(td => {
                    row.push(td.textContent);    
                });
                data.rows.push(row);
            });
            return JSON.stringify(data);
        }

        blockElement.addRow = function() {
            const row = document.createElement('tr');
            const cols = this.querySelectorAll('th').length || 3;
            for(let c=0; c<cols; c++) {
                const td = document.createElement('td');
                td.classList.add('editable');
                td.setAttribute('contenteditable', 'true');
                td.textContent = `Data ${this.querySelectorAll('tr').length+1}-${c+1}`;
                td.addEventListener('blur', (e) => {
                    if(td.innerHTML !== td.dataset.value) {
                        this.setAttribute("dirty", true);
                    }
                });
                row.appendChild(td);
            }
            this.querySelector('tbody').appendChild(row);
            this.setAttribute("dirty", true);
        }

        blockElement.addColumn = function() {
            const th = document.createElement('th');
            th.classList.add('editable');
            th.setAttribute('contenteditable', 'true');
            th.textContent = `Header ${this.querySelectorAll('th').length+1}`;
            th.addEventListener('blur', (e) => {
                if(th.innerHTML !== th.dataset.value) {
                    this.setAttribute("dirty", true);
                }
            });
            this.querySelector('thead tr').appendChild(th);
            this.querySelectorAll('tr').forEach((tr, i) => {
                const td = document.createElement('td');
                td.classList.add('editable');
                td.setAttribute('contenteditable', 'true');
                td.textContent = `Data ${i+1}-${this.querySelectorAll('th').length}`;
                td.addEventListener('blur', (e) => {
                    if(td.innerHTML !== td.dataset.value) {
                        this.setAttribute("dirty", true);
                    }
                });
                tr.appendChild(td);
            }
            );
            this.setAttribute("dirty", true);
        }
    }

    static createDummyTable(element, data) {
        this.buildTable(element, data);
        // Build 3x3 table with no content
        let table = `<table class="table-striped">`;
        if(data.headers){ 
            table += '<thead><tr>';
            for(let c=0; c<cols; c++) {
                table += `<th class="editable" contenteditable>Header ${c+1}</th>`;
            }
            table += '</tr></thead>';
        }
        table += '<tbody>';
        for(let r=0; r<rows; r++) {
            table += '<tr>';
            for(let c=0; c<cols; c++) {
                table += `<td class="editable" contenteditable>Data ${r+1}-${c+1}</td>`;
            }
            table += '</tr>';
        }
        table += '</tbody></table>';
        element.innerHTML = table;
        element.dataset.dirty = false;
        // Make all cells editable
        const parent = element;
        parent.querySelectorAll('.editable').forEach(element => {
            element.dataset.value = element.innerHTML;
            element.addEventListener('blur', (e) => {
                if(element.innerHTML !== element.dataset.value) {
                    parent.setAttribute("dirty", true);
                }
            });
        });
    }

    static loadTable(element, content) {
        let data = JSON.parse(content || {});
        this.buildTable(element, data);

        let table = `<table class="${data.style || 'table-striped'}">`;
        if(data.headers){ // && data.headers === 'true') {
            table += '<thead><tr>';
            for(let header of data.headers) {
                table += `<th class="editable" contenteditable>${header}</th>`;
            }
            table += '</tr></thead>';
        }
        table += '<tbody>';
        for(let row of data.rows) {
            table += '<tr>';
            for(let col of row) {
                table += `<td class="editable" contenteditable>${col}</td>`;
            }
            table += '</tr>';
        }
        table += '</tbody></table>';
        element.innerHTML = table;
        element.dataset.dirty = false;
        const parent = element;
        parent.querySelectorAll('.editable').forEach(element => {
            // el.attributes.contenteditable = true;
            element.dataset.value = element.innerHTML;
            element.addEventListener('input', (e) => {
                parent.setAttribute("dirty", true);
            });

            element.addEventListener('blur', (e) => {
                if(element.innerHTML !== element.dataset.value) {
                    parent.setAttribute("dirty", true);
                }
            });
        });
    }
    
}
