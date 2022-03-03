import { make_polygon, polygon_contains } from '@bounce/physics';
import { init, loop, scope } from './canvas2d';

const ctx = init();

const points = [
  [30, 40],
  [40, -10],
  [-10, -20],
  [-50, -15],
  [-30, 30],
];

// const poly = make_polygon(new Float32Array(points.flat()));

let a = 0;
let va = 1;
let o = 0.1;

window.addEventListener('mousemove', (e) => {
  const { width, height } = document.body.getBoundingClientRect();
  va = Math.expm1(e.clientX / width) * 5;
  o = (Math.expm1(e.clientY / height) / (Math.E - 1)) ** 2;
});

let t = performance.now();

const PHI = (1 + Math.sqrt(5)) / 2;

loop(() => {
  const t2 = performance.now();
  const dt = (t2 - t) / 1000;
  t = t2;

  a += va * dt;

  ctx.fillStyle = `rgba(${o * 255} ${o * 255} ${o * 255})`;
  ctx.globalCompositeOperation = 'lighter';
  ctx.fillRect(-1000, -1000, 2000, 2000);
  ctx.globalCompositeOperation = 'source-over';
  const h = Math.sin(a / 10) * 180 + 180;
  const s = Math.sin((a / 10) * PHI) * 25 + 75;
  const l = Math.sin(a / 10 / PHI) * 25 + 50;
  ctx.strokeStyle = `hsl(${h} ${s}% ${l}%)`;
  // ctx.fillText(`${va.toPrecision(2)} ${o.toPrecision(2)}`, 0, 0);
  scope(ctx, () => {
    ctx.rotate(a);
    ctx.translate(0, Math.sin((a / 100) * PHI ** 2) * 20 + 50);
    ctx.scale(10, 10);
    ctx.rotate(PHI ** 3 * a);
    ctx.translate(-0.5, -0.5);
    ctx.lineWidth = 0.1;
    ctx.strokeRect(0, 0, 1, 1);
  });
});
