import {
  make_polygon,
  polygon_draw,
  polygon_intersection,
} from '@bounce/physics';
import { init, loop, scope } from './canvas2d';

const [canvas, ctx] = init();

const points = [
  [30, 40],
  [40, -10],
  [-20, -20],
  [-30, -15],
  [-30, 30],
];

const poly = make_polygon(new Float32Array(points.flat()));

let t = performance.now();

let cmx = 0;
let cmy = 0;

const log: string[] = [];

document.addEventListener('click', (e) => {
  cmx = e.clientX;
  cmy = e.clientY;
  canvas.requestPointerLock();
});

document.addEventListener('mousemove', (e) => {
  cmx += e.movementX;
  cmy += e.movementY;
});

loop(() => {
  const t2 = performance.now();
  const dt = (t2 - t) / 1000;
  t = t2;

  const imatrix = ctx.getTransform().inverse();

  const mouse = imatrix.transformPoint(new DOMPoint(cmx, cmy));

  ctx.fillStyle = '#fff';
  ctx.fillRect(-1000, -1000, 2000, 2000);
  ctx.strokeStyle = 'solid black 2px';
  polygon_draw(poly, ctx);
  const p = polygon_intersection(poly, mouse.x, mouse.y);
  ctx.fillStyle = '#444';
  if (p) ctx.fill();
  ctx.stroke();
  if (p) {
    ctx.beginPath();
    ctx.moveTo(p.x, p.y);
    ctx.lineTo(mouse.x, mouse.y);
    ctx.stroke();
  }
  ctx.fillStyle = 'red';
  scope(ctx, () => {
    ctx.translate(mouse.x, mouse.y);
    ctx.beginPath();
    ctx.ellipse(0, 0, 2, 2, 0, 0, Math.PI * 2);
    ctx.fill();
  });
});
