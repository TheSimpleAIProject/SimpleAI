// A script that is applied to the OnnxDragIn with a serach bar input and a container of nodes.
// It offers search capabilities and draggable Nodes with can be added to the OnnxViewport.
// This only works with when the window.activeOnnxViewport is set.
// This approach offers better readablily as opposed to all java script.
// Also this could be replaced eventually with a native rendering solution from dixous
class OnnxDragIn {
  constructor(rootelm, search, nodeContainer) {
    this.rootelm = rootelm;
    this.search = search;
    this.nodeContainer = nodeContainer;
    this.draggingChild = null;

    this._ondrag = this._ondrag.bind(this);
    this._ondrop = this._ondrop.bind(this);

    this.nodeContainer.addEventListener("mousedown", (e) => {
      let child = e.target.closest(".draggable-node");
      if (!child || !this.nodeContainer.contains(child)) return;

      this.draggingChild = child;
      this.draggingChild.style.position = "fixed";
      window.addEventListener("mousemove", this._ondrag);
    });

    window.addEventListener("mouseup", (e) => {
      if (this.draggingChild) {
        window.removeEventListener("mousemove", this._ondrag);
        this._ondrop(e);
      }
    });
  }

  _ondrag(e) {
    let rect = this.draggingChild.getBoundingClientRect();
    this.draggingChild.style.left = `${e.x - rect.width / 2}px`;
    this.draggingChild.style.top = `${e.y - rect.height / 2}px`;
  }

  _ondrop(e) {
    this.draggingChild.style.position = "initial";
    this.draggingChild.style.left = "initial";
    this.draggingChild.style.top = "initial";
    window.activeOnnxViewport.addNodeFromId(this.draggingChild.id, {
      position: { x: e.x, y: e.y },
    });
    this.draggingChild = null;
  }
}

window.OnnxDragIn = OnnxDragIn;
