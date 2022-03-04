import './app.css';

export const init = () => {
  const canvas = document.createElement('canvas');

  document.body.appendChild(canvas);

  const ctx = canvas.getContext('2d');

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
  return [canvas, ctx] as const;
};

export const loop = (block: () => unknown) => {
  const doLoop = () => {
    block();
    window.requestAnimationFrame(doLoop);
  };

  window.requestAnimationFrame(doLoop);
  // setInterval(block, 500);
};
