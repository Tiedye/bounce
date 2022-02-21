import './app.css';

const canvas = document.createElement('canvas');

document.body.appendChild(canvas);

const ctx = canvas.getContext('2d');

const scope = (
  ctx: { save: () => unknown; restore: () => unknown },
  block: () => unknown
) => {
  ctx.save();
  block();
  ctx.restore();
};

if (!ctx) throw new Error('no context');

const fixSize = () => {
  const { width, height } = document.body.getBoundingClientRect();
  canvas.width = width;
  canvas.height = height;
  const scale = Math.min(width, height) / 200;
  ctx.resetTransform();
  ctx.scale(scale, scale);
  ctx.translate(width / scale / 2, height / scale / 2);
};

window.addEventListener('resize', fixSize);
fixSize();

let a = 0;
let va = 1;
let o = 0.1;

window.addEventListener('mousemove', (e) => {
  const { width, height } = document.body.getBoundingClientRect();
  va = Math.expm1(e.clientX / width) * 5;
  o = (Math.expm1(e.clientY / height) / (Math.E - 1)) ** 2;
});

let t = performance.now();

const render = () => {
  const t2 = performance.now();
  const dt = (t2 - t) / 1000;
  t = t2;

  a += va * dt;

  ctx.fillStyle = `rgba(255 255 255 / ${o})`;
  ctx.fillRect(-1000, -1000, 2000, 2000);
  ctx.fillStyle = '#000';
  // ctx.fillText(`${va.toPrecision(2)} ${o.toPrecision(2)}`, 0, 0);
  scope(ctx, () => {
    ctx.rotate(a);
    ctx.translate(0, 50);
    ctx.scale(10, 10);
    ctx.rotate(10 * a);
    ctx.translate(-0.5, -0.5);
    ctx.lineWidth = 0.1;
    ctx.strokeRect(0, 0, 1, 1);
  });
};

const loop = () => {
  render();
  window.requestAnimationFrame(loop);
};

window.requestAnimationFrame(loop);
