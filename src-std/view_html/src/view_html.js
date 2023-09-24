const template = document.createElement("template");
template.innerHTML = `
<div id="display"></div>
`;

export class NodeFrontEnd extends HTMLElement {
  constructor() {
    super();
    this.shadow = this.attachShadow({ mode: "open" });
    this.shadowRoot.appendChild(template.content.cloneNode(true));
  }

  static get observedAttributes() {
    return ["data", "data_callback"];
  }

  attributeChangedCallback(property, oldValue, newValue) {
    if (oldValue === newValue) return;
    this[property] = newValue;

    const const_text = this.shadowRoot.getElementById('display');
    const_text.innerHTML = '';
    const_text.innerHTML = this.data;
  }

  connectedCallback() {
    const filterFloat = function (value) {
      if(/^(\-|\+)?([0-9]+(\.[0-9]+)?|Infinity)$/
        .test(value))
      return Number(value);
      return NaN;
    };
    const const_text = this.shadowRoot.getElementById('display');
    const_text.insertAdjacentHTML("afterbegin", this.data);
    const_text.oninput  = () => {this.data_callback({value: (isNaN(filterFloat(const_text.value))?const_text.value:filterFloat(const_text.value))})};
  }
}
