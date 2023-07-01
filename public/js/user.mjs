import { useContext } from "./context.mjs";
import { loadUser } from "./service.mjs";

export class UserElement extends HTMLElement {
    static get observedAttributes() {
        return ['user'];
    }

    connectedCallback() {
        const userContext = useContext('user');
        const userString = localStorage.getItem('user');
        if(userString) {
            this.user = JSON.parse(userString);
            userContext.set('user', this.user);
        }
        this.show();
    }

    async loginUser(){
        const userContext = useContext('user');
        const pageContext = useContext('page');
        
        const user = await loadUser(this.username, this.password);
        if(user){
            this.user = user;
            userContext.set('user', user);
            localStorage.setItem('user', JSON.stringify(user));
            if(user.page_version_id) pageContext.set('page', user.page_version_id);
        }
        return user;
    }
    
    show(){
        console.log('show', this.isLoggedin());
        this.innerHTML = this.isLoggedin() ? `<div>Username: ${this.user.name} <button id="logout" >Logout</button></div>` :
            `<div class="login"><div><span>Username</span><input id="username"></input></div><div><span>Password</span><input id="password"></input></div><button id="login" >Login</button></div>`;    

        const loginButton = document.getElementById('login');
        if(loginButton) {
            loginButton.addEventListener('click', () => {
                console.log('loginButton click');
                (async () => { 
                    let user = await this.loginUser();
                    this.user = user;
                    this.show();})();
                
            });
        }            
        const logoutButton = document.getElementById('logout');
        if(logoutButton) {
            logoutButton.addEventListener('click', () => {
                console.log('logoutButton click');
                this.user = null;
                localStorage.removeItem('user');
                this.show();                
            });
        }            
        const usernameInput = document.getElementById('username');
        if(usernameInput) {
            usernameInput.addEventListener('change', (e) => {
                console.log('usernameInput change', e.target.value);
                this.username = e.target.value;
            });
        }
        const passwordInput = document.getElementById('password');
        if(passwordInput) {
            passwordInput.addEventListener('change', (e) => {              
                this.password = e.target.value;
            });
        }
    }

    isLoggedin() {
        return !!this.user;
    }

    async attributeChangedCallback(name, oldValue, newValue) {
        if(name === 'user') {
            this.user = newValue
            this.show();
        }
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

