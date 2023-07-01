export class Context {
    constructor() {
        this.context = {}
    }

    set(key, value) {
        this.context[key] = value
        handleContextChange("--NEW--", this.context, key, null, value);
    }

    get(key) {
        let value = this.context[key];
        if(!value) {
            value = new SubContext(key);
            this.context[key] = value;
        }
        return value;
    }
};

export class SubContext {
    constructor(name) {
        this.context = {}
        this.name = name;
    }

    set(key, value) {
        const oldValue = this.context[key]
        this.context[key] = value
        handleContextChange(this.name, this.context, key, oldValue, value);
    }

    get(key) {
        return this.context[key];
    }
};

function handleContextChange(name, context, elementName, elementOldValue, elementNewValue) {
    document.dispatchEvent(
        new CustomEvent('context', {
            bubbles: true,
            composed: true,
            detail: {
                name: name,
                data: context,
                element: {
                    name: elementName,
                    oldValue: elementOldValue,
                    newValue: elementNewValue, 
                }
            },
        })
    );
}

const context = new Context();

export function useContext(region){
    return context.get(region);
}

export function setContext(html, context){
    html.setAttribute('context', context);
}

export class ContextElement extends HTMLElement {
    connectedCallback() {
        // const body = self.innerHTML;
        // self.innerHTML = `${body}</ul>`;    
    }
}
