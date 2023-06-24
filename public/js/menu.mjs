export class NavMenuElement extends HTMLElement {
    connectedCallback() {
        this.classList.add("menu");
        const body = self.innerHTML;
        self.innerHTML = `<ul>${body}</ul>`;    
    }

    handleItemClick(e, id, dataset) {
        this.classList.toggle('toggle__closed');
        this.dispatchEvent(
            new CustomEvent('action', {
                bubbles: true,
                composed: true,
                detail: {
                    id: id,
                    data: dataset,
                },
            })
        );
    }
}

export class NavMenuItemElement extends HTMLElement {
    handleClick(e) {
        const element = this.parentElement;
        //console.log('NavMenuItemElement handleClick', element);
        element.handleItemClick(e, this.id, this.dataset);
        e.stopPropagation();
        e.preventDefault();
    }
    connectedCallback() {
        const self = this;
        if(this.attributes.init) return;
        this.setAttribute('init', true);
        const id = this.id;
        const body = self.innerHTML;
        const title = this.attributes.title ? this.attributes.title.value : body;
        //console.log('NavMenuItemElement connectedCallback',title);
        const type = this.attributes.type ? this.attributes.type.value : 'link';
        const op = this.dataset.operation.value
        const li = document.createElement('li');
        li.dataset.id = id;
        li.classList.add('menu__link');
        li.dataset.op = op;
        li.dataset.type = type;
        li.innerHTML = title;
        this.innerHTML = null;
        this.appendChild(li);
        this.addEventListener('click', (e) => {this.handleClick(e); e.stopPropagation(); e.preventDefault();});    
    }
}

export class NavSubMenuElement extends HTMLElement {
    connectedCallback() {
        const self = this;
        const id = this.id;
        const type = this.attributes.type ? this.attributes.type.value : 'none';
        const title = this.attributes.title ? this.attributes.title.value : 'none';
        const body = self.innerHTML;
        self.innerHTML = `<li class="parent__menu" data-sub-menu="menu-style-submenu">${title}</li>
        <ul id="${id}-submenu" class="sub__menu toggle__closed">${body}</ul>`;
        this.addEventListener('click', (e) => {
            //console.log('NavSubMenuElement handleClick', e, id);
            document.getElementById(`${id}-submenu`).classList.toggle('toggle__closed'); e.stopPropagation(); e.preventDefault();});    
    }

    handleItemClick(e, id, dataset) {
        const element = this.parentElement;
        //console.log('NavSubMenuElement handleItemClick', id, dataset, element);
        element.handleItemClick(e, id, dataset);
        e.stopPropagation();
        e.preventDefault();
    }

}
