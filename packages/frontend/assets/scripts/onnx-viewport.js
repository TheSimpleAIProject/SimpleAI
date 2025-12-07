/**
 * A reusable, instantiable node-editor viewport module.
 * Usage:
 *   import { Viewport, VNode, Parameter, Connection } from './viewport.js';
 *   const vp = new Viewport(document.getElementById('editor'));
 *   vp.addNode(new VNode(100,100,'A',[new Parameter('output','outA')]));
 *  ... etc.
 **/

function drawRoundedRect(ctx, x, y, width, height, radius = 6, options = {}) {
  // radius can be a number or an object {tl, tr, br, bl}
  const r =
    typeof radius === "number"
      ? { tl: radius, tr: radius, br: radius, bl: radius }
      : Object.assign({ tl: 0, tr: 0, br: 0, bl: 0 }, radius);

  // clamp radii to not exceed half width/height
  const maxR = Math.min(width, height) / 2;
  r.tl = Math.min(r.tl, maxR);
  r.tr = Math.min(r.tr, maxR);
  r.br = Math.min(r.br, maxR);
  r.bl = Math.min(r.bl, maxR);

  const {
    fill = true,
    stroke = false,
    fillStyle,
    strokeStyle,
    lineWidth = 1,
  } = options;

  ctx.beginPath();
  ctx.moveTo(x + r.tl, y);
  ctx.lineTo(x + width - r.tr, y);
  ctx.quadraticCurveTo(x + width, y, x + width, y + r.tr);
  ctx.lineTo(x + width, y + height - r.br);
  ctx.quadraticCurveTo(x + width, y + height, x + width - r.br, y + height);
  ctx.lineTo(x + r.bl, y + height);
  ctx.quadraticCurveTo(x, y + height, x, y + height - r.bl);
  ctx.lineTo(x, y + r.tl);
  ctx.quadraticCurveTo(x, y, x + r.tl, y);
  ctx.closePath();

  if (fill) {
    if (fillStyle !== undefined) ctx.fillStyle = fillStyle;
    ctx.fill();
  }

  if (stroke) {
    if (strokeStyle !== undefined) ctx.strokeStyle = strokeStyle;
    ctx.lineWidth = lineWidth;
    ctx.stroke();
  }
}

class VNodeParameter {
  constructor(type = "input", name = "") {
    this.type = type; // "input" or "output"
    this.name = name;
    // visual defaults (can be overridden externally)
    this.radius = 10;
    this.fillInput = "#48e";
    this.fillOutput = "#fa3";
    this.textColor = "#fff";
  }

  // draw the parameter socket and its label. `pos` is {x,y}.
  draw(ctx, pos, isInput) {
    ctx.beginPath();
    ctx.arc(pos.x, pos.y, this.radius, 0, 2 * Math.PI);
    ctx.fillStyle = isInput ? this.fillInput : this.fillOutput;
    ctx.fill();
    // ctx.stroke();

    ctx.fillStyle = this.textColor;
    // Labels are drawn to the right for inputs, to the left for outputs
    if (isInput) {
      ctx.fillText(this.name, pos.x + 16, pos.y + 6);
    } else {
      ctx.fillText(this.name, pos.x - 60, pos.y + 6);
    }
  }
}

class VNode {
  constructor(x = 0, y = 0, label = "Node", params = []) {
    this.label = label;
    // params is array of Parameter instances or plain objects {type,name}
    this.params = params.map((p) =>
      p instanceof VNodeParameter ? p : new VNodeParameter(p.type, p.name),
    );
    this.inputs = this.params.filter((p) => p.type === "input");
    this.outputs = this.params.filter((p) => p.type === "output");

    // visual defaults
    this.paramSpacing = 50;
    this.width = 160;
    this.height = Math.max(
      48,
      32 +
        Math.max(this.inputs.length, this.outputs.length) * this.paramSpacing,
    );
    this.x = x - this.width / 2;
    this.y = y - this.height / 2;
    this.fillStyle = "#193c4d";
    this.strokeStyle = "#58a";
    this.titleColor = "#fff";
    this.titleFont = "16px sans-serif";
    this.heading_space = 60;
  }

  // compute input socket coordinates
  getInputCoords(i) {
    return {
      x: this.x,
      y: this.y + this.heading_space + i * this.paramSpacing,
    };
  }
  // compute output socket coordinates
  getOutputCoords(i) {
    return {
      x: this.x + this.width,
      y: this.y + this.heading_space + i * this.paramSpacing,
    };
  }

  contains(px, py) {
    return (
      px > this.x &&
      px < this.x + this.width &&
      py > this.y &&
      py < this.y + this.height
    );
  }

  inputHit(px, py) {
    for (let i = 0; i < this.inputs.length; ++i) {
      let p = this.getInputCoords(i);
      if (Math.hypot(px - p.x, py - p.y) < 14) return i;
    }
    return null;
  }

  outputHit(px, py) {
    for (let i = 0; i < this.outputs.length; ++i) {
      let p = this.getOutputCoords(i);
      if (Math.hypot(px - p.x, py - p.y) < 14) return i;
    }
    return null;
  }

  // snap size to gridSpacing
  fitToGrid(gridSpacing) {
    this.width = Math.ceil(this.width / gridSpacing) * gridSpacing;
    this.height = Math.ceil(this.height / gridSpacing) * gridSpacing;
  }

  // draw the whole node including its parameters (uses Parameter.draw)
  draw(ctx, viewport) {
    // Ensure node size aligns to grid for consistent visuals
    this.fitToGrid(viewport.gridSpacing);

    ctx.fillStyle = this.fillStyle;
    // ctx.strokeStyle = this.strokeStyle;
    // ctx.lineWidth = 2;
    // drawLiquidGlassRect(ctx, this.x, this.y, this.width, this.height);
    drawRoundedRect(ctx, this.x, this.y, this.width, this.height, 15);
    // ctx.fillRect(this.x, this.y, this.width, this.height);
    // ctx.strokeRect(this.x, this.y, this.width, this.height);

    ctx.fillStyle = this.titleColor;
    ctx.font = this.titleFont;
    ctx.fillText(this.label, this.x + 10, this.y + 22);

    // draw inputs (left)
    this.inputs.forEach((input, i) => {
      const p = this.getInputCoords(i);
      input.draw(ctx, p, true);
    });

    // draw outputs (right)
    this.outputs.forEach((output, i) => {
      const p = this.getOutputCoords(i);
      output.draw(ctx, p, false);
    });
  }
}

class VConnection {
  constructor(fromNode, fromOutput, toNode, toInput) {
    this.fromNode = fromNode;
    this.fromOutput = fromOutput;
    this.toNode = toNode;
    this.toInput = toInput;

    // visual defaults
    this.color = "#fb0";
    this.width = 3;
  }

  hit(px, py) {
    // Simple hit test: closest approach to Bezier curve
    // We'll use a simple linear approach for now for demo (improve if accuracy needed)
    const from = this.fromNode.getOutputCoords(this.fromOutput);
    const to = this.toNode.getInputCoords(this.toInput);
    // test a few points along the curve
    for (let t = 0; t <= 1; t += 0.05) {
      let x =
        Math.pow(1 - t, 3) * from.x +
        3 * Math.pow(1 - t, 2) * t * (from.x + 40) +
        3 * (1 - t) * Math.pow(t, 2) * (to.x - 40) +
        Math.pow(t, 3) * to.x;
      let y =
        Math.pow(1 - t, 3) * from.y +
        3 * Math.pow(1 - t, 2) * t * from.y +
        3 * (1 - t) * Math.pow(t, 2) * to.y +
        Math.pow(t, 3) * to.y;
      if (Math.hypot(px - x, py - y) < 10) return true;
    }
    return false;
  }
  draw(ctx /*, viewport - not required here but kept for parity */) {
    const from = this.fromNode.getOutputCoords(this.fromOutput);
    const to = this.toNode.getInputCoords(this.toInput);
    ctx.strokeStyle = this.color;
    ctx.lineWidth = this.width;
    ctx.beginPath();
    ctx.moveTo(from.x, from.y);
    ctx.bezierCurveTo(from.x + 40, from.y, to.x - 40, to.y, to.x, to.y);
    ctx.stroke();
  }
}

class Viewport {
  constructor(dioxus, containerElement, options = {}) {
    if (!(containerElement instanceof HTMLElement)) {
      throw new Error("Viewport constructor requires a DOM container element");
    }

    this.dioxus = dioxus;

    // options and defaults
    this.gridSpacing = options.gridSpacing || 40;
    this.dotRadius = options.dotRadius || 2;
    this.minZoom = options.minZoom || 0.4;
    this.maxZoom = options.maxZoom || 2.5;

    // internal state
    this.zoom = options.zoom || 1;
    this.offsetX = options.offsetX || 0;
    this.offsetY = options.offsetY || 0;

    this.nodes = [];
    this.connections = [];

    this.mouse = { x: 0, y: 0 };
    this.draggingNode = null;
    this.dragOffsetX = 0;
    this.dragOffsetY = 0;
    this.connectingFrom = null; // { node, outIdx }
    this.isPanning = false;
    this.panStart = { x: 0, y: 0 };
    this.panOrigin = { x: 0, y: 0 };

    // create canvas and append to container
    this.container = containerElement;
    this.canvas = document.createElement("canvas");
    this.canvas.style.width = "100%";
    this.canvas.style.height = "100%";
    this.canvas.style.display = "block";
    this.container.appendChild(this.canvas);
    this.ctx = this.canvas.getContext("2d");

    this.selectedNode = null;
    this.selectedConnection = null;

    // bind methods
    this._onResize = this._onResize.bind(this);
    this._onMouseDown = this._onMouseDown.bind(this);
    this._onMouseMove = this._onMouseMove.bind(this);
    this._onMouseUp = this._onMouseUp.bind(this);
    this._onWheel = this._onWheel.bind(this);
    this._onKeyDown = this._onKeyDown.bind(this);

    // event listeners
    this._containerResizeObserver = new ResizeObserver(this._onResize);
    this._containerResizeObserver.observe(this.container);
    this.canvas.addEventListener("mousedown", this._onMouseDown);
    this.canvas.addEventListener("mousemove", this._onMouseMove);
    this.canvas.addEventListener("mouseup", this._onMouseUp);
    this.canvas.addEventListener("wheel", this._onWheel, { passive: false });
    document.addEventListener("keydown", this._onKeyDown);

    // initial sizing and draw
    this._onResize();
  }

  addNode(node) {
    this.nodes.push(node);
    this.draw();
  }

  addNodeFromId(id, options = {}) {
    let rect = this.container.getBoundingClientRect();
    const {
      position = { x: rect.width / 2 + rect.x, y: rect.height / 2 + rect.y },
    } = options;

    let { x, y } = this.toEditor(position.x - rect.x, position.y - rect.y);

    this.dioxus.send({
      AddNode: { id: "00000000-0000-0000-0000-000000000000", x: x, y: y },
    });
  }

  handleAddNode(jsonNode) {
    let node = new VNode(
      jsonNode.x,
      jsonNode.y,
      jsonNode.label,
      jsonNode.params,
    );
    console.log(node);
    this.addNode(node);
  }

  save() {
    return { nodes: this.nodes, connections: this.connections };
  }

  addConnection(conn) {
    this.connections.push(conn);
    this.draw();
  }

  clear() {
    this.nodes = [];
    this.connections = [];
    this.draw();
  }

  // convert canvas pixel coords to editor coordinates (considering transform)
  toEditor(x, y) {
    return {
      x: (x - this.offsetX) / this.zoom,
      y: (y - this.offsetY) / this.zoom,
    };
  }

  // resizing helper
  _onResize() {
    const rect = this.container.getBoundingClientRect();
    this.canvas.width = rect.width;
    this.canvas.height = rect.height;
    this.draw();
  }

  // central draw method for the viewport (required)
  draw() {
    const ctx = this.ctx;
    // reset transform & clear
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
    // apply zoom + pan transform
    ctx.setTransform(this.zoom, 0, 0, this.zoom, this.offsetX, this.offsetY);

    // Grid dots
    ctx.fillStyle = "#3a3c40";
    const w = (this.canvas.width - this.offsetX) / this.zoom;
    const h = (this.canvas.height - this.offsetY) / this.zoom;
    const startX =
      Math.floor(-this.offsetX / this.zoom / this.gridSpacing) *
      this.gridSpacing;
    const startY =
      Math.floor(-this.offsetY / this.zoom / this.gridSpacing) *
      this.gridSpacing;
    for (let x = startX; x < w + this.gridSpacing; x += this.gridSpacing) {
      for (let y = startY; y < h + this.gridSpacing; y += this.gridSpacing) {
        ctx.beginPath();
        ctx.arc(x, y, this.dotRadius, 0, 2 * Math.PI);
        ctx.fill();
      }
    }

    // Live connect preview
    if (this.connectingFrom) {
      const from = this.connectingFrom.node.getOutputCoords(
        this.connectingFrom.outIdx,
      );
      ctx.strokeStyle = "#fb0";
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.moveTo(from.x, from.y);
      ctx.bezierCurveTo(
        from.x + 40,
        from.y,
        this.mouse.x - 40,
        this.mouse.y,
        this.mouse.x,
        this.mouse.y,
      );
      ctx.stroke();
    }

    // Connections
    for (let c of this.connections) {
      if (c === this.selectedConnection) {
        // Highlight selected connection
        this.ctx.save();
        this.ctx.shadowColor = "#fb0";
        this.ctx.shadowBlur = 8;
        c.draw(this.ctx, this);
        this.ctx.restore();
        // Optional: thicker
        this.ctx.save();
        this.ctx.strokeStyle = "#f00";
        this.ctx.lineWidth = 7;
        c.draw(this.ctx, this);
        this.ctx.restore();
      } else {
        c.draw(this.ctx, this);
      }
    }

    // VNodes
    for (let n of this.nodes) {
      n.draw(this.ctx, this);
      if (n === this.selectedNode) {
        // Highlight selected node
        this.ctx.save();
        // this.ctx.strokeStyle = "#fb0";
        // this.ctx.lineWidth = 7;

        this.ctx.strokeStyle = "#fb0";
        drawRoundedRect(this.ctx, n.x, n.y, n.width, n.height, 19, {
          stroke: true,
          fill: false,
        });
        this.ctx.restore();
      }
    }

    // reset transform for any overlay drawing in screen space if desired
    ctx.setTransform(1, 0, 0, 1, 0, 0);
  }

  // -------- Input handlers ----------
  _onMouseDown(e) {
    const p = this.toEditor(e.offsetX, e.offsetY);
    this.mouse = p;

    // output hit first (for drag to input)
    for (let node of this.nodes) {
      let outIdx = node.outputHit(p.x, p.y);
      if (outIdx !== null) {
        this.connectingFrom = { node, outIdx };
        e.preventDefault();
        this.selectedNode = null;
        this.selectedConnection = null;
        this.draw();
        return;
      }
    }

    // check connections first for hit
    for (let i = 0; i < this.connections.length; ++i) {
      if (this.connections[i].hit(p.x, p.y)) {
        this.selectedConnection = this.connections[i];
        this.selectedNode = null;
        this.draw();
        return;
      }
    }

    // hit test nodes (dragging/select)
    for (let node of this.nodes) {
      if (node.contains(p.x, p.y)) {
        this.draggingNode = node;
        this.dragOffsetX = p.x - node.x;
        this.dragOffsetY = p.y - node.y;
        this.selectedNode = node;
        this.selectedConnection = null;
        this.draw();
        return;
      }
    }

    // start panning
    this.isPanning = true;
    this.panOrigin.x = this.offsetX;
    this.panOrigin.y = this.offsetY;
    this.panStart.x = e.offsetX;
    this.panStart.y = e.offsetY;
    this.selectedNode = null;
    this.selectedConnection = null;
    this.draw();
  }

  _onMouseMove(e) {
    const p = this.toEditor(e.offsetX, e.offsetY);
    this.mouse = p;
    if (this.draggingNode) {
      let newX =
        Math.round((p.x - this.dragOffsetX) / this.gridSpacing) *
        this.gridSpacing;
      let newY =
        Math.round((p.y - this.dragOffsetY) / this.gridSpacing) *
        this.gridSpacing;
      this.draggingNode.x = newX;
      this.draggingNode.y = newY;
      this.draggingNode.fitToGrid(this.gridSpacing);
      this.draw();
    } else if (this.isPanning) {
      this.offsetX = this.panOrigin.x + (e.offsetX - this.panStart.x);
      this.offsetY = this.panOrigin.y + (e.offsetY - this.panStart.y);
      this.draw();
    } else if (this.connectingFrom) {
      this.draw();
    }
  }

  _onMouseUp(e) {
    const p = this.toEditor(e.offsetX, e.offsetY);
    if (this.draggingNode) {
      this.draggingNode = null;
      this.draw();
    } else if (this.connectingFrom) {
      for (let node of this.nodes) {
        let inIdx = node.inputHit(p.x, p.y);
        if (inIdx !== null && node !== this.connectingFrom.node) {
          this.connections.push(
            new VConnection(
              this.connectingFrom.node,
              this.connectingFrom.outIdx,
              node,
              inIdx,
            ),
          );
          break;
        }
      }
      this.connectingFrom = null;
      this.draw();
    } else if (this.isPanning) {
      this.isPanning = false;
      this.draw();
    }
  }

  _onWheel(e) {
    let scale = 1 + (e.deltaY < 0 ? 0.1 : -0.1);
    let mx = e.offsetX,
      my = e.offsetY;
    const before = this.toEditor(mx, my);
    this.zoom = Math.max(
      this.minZoom,
      Math.min(this.maxZoom, this.zoom * scale),
    );
    const after = this.toEditor(mx, my);
    // Adjust offsets so that the point under the mouse stays stationary in editor space
    this.offsetX += (after.x - before.x) * this.zoom;
    this.offsetY += (after.y - before.y) * this.zoom;
    this.draw();
    e.preventDefault();
  }

  _onKeyDown(e) {
    if (e.key === "Delete" || e.key === "Backspace") {
      let removed = false;
      if (this.selectedNode) {
        // Remove node and its connections
        const idx = this.nodes.indexOf(this.selectedNode);
        if (idx !== -1) {
          this.nodes.splice(idx, 1);
          // Remove connections attached to this node
          this.connections = this.connections.filter(
            (c) =>
              c.fromNode !== this.selectedNode &&
              c.toNode !== this.selectedNode,
          );
          removed = true;
        }
        this.selectedNode = null;
      }
      if (this.selectedConnection) {
        const idx = this.connections.indexOf(this.selectedConnection);
        if (idx !== -1) {
          this.connections.splice(idx, 1);
          removed = true;
        }
        this.selectedConnection = null;
      }
      if (removed) {
        this.draw();
        e.preventDefault();
      }
    }
  }

  destroy() {
    // window.removeEventListener("resize", this._onResize);
    this.canvas.removeEventListener("mousedown", this._onMouseDown);
    this.canvas.removeEventListener("mousemove", this._onMouseMove);
    this.canvas.removeEventListener("mouseup", this._onMouseUp);
    this.canvas.removeEventListener("wheel", this._onWheel);
    document.removeEventListener("keydown", this._onKeyDown);
    if (this.canvas.parentElement) {
      this.canvas.parentElement.removeChild(this.canvas);
    }
  }
}

window.Parameter = VNodeParameter;
window.VNode = VNode;
window.Viewport = Viewport;
