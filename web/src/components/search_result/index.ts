import { reg_css, fetch_html } from "scripts";
reg_css("search-result", "sr");

// @ts-ignore
const invoke = window.__TAURI__.invoke; // uncomment
export class SearchResult extends HTMLElement
{
    constructor()
    {
        super();
        fetch_html(
            this,
            'components/search_result/index.html',
            () => {
                // this.shadowRoot.querySelector('#name').innerHTML = node.name;
                // this.shadowRoot.querySelector('#version').innerHTML = node.version;
                // this.shadowRoot.querySelector('#website').innerHTML = node.website;
                // this.shadowRoot.querySelector('#description').innerHTML = node.description;
                // this.shadowRoot.querySelector('#author').innerHTML = node.author;
                let favourite: Element = this.shadowRoot.querySelector('#favorite');
                favourite.addEventListener('click', () => {
                    // invoke('toggle_favourite', {id: this.nodeId});
                    // if (invoke('is_favourite') {
                    favourite.className = "ri-star-fill";
                    // }
                    // else
                    // {
                    //      favourite.className = "ri-star-line";
                    // }
                });
                let open: Element = this.shadowRoot.querySelector('#open');
                open.addEventListener('click', () => {
                    console.log("opening editor...")
                    invoke('editor_page'); //{id: node.default_hash()});
                });
            });
    }
}

customElements.define('search-result', SearchResult);
console.log("script loaded.")