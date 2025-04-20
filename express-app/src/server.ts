import express from 'express';
import promClient, { PrometheusContentType } from 'prom-client';

const app = express();
const port = 8082;

const register = new promClient.Registry();

promClient.collectDefaultMetrics({
  register: register,
});
const summary = new promClient.Summary({
  registers: [register],
  help: 'Http Server request seconds',
  name: 'http_server_requests_seconds',
  labelNames: ['error', 'exception', 'method', 'outcome', 'status', 'uri'],
});
app.use((req, res, next) => {
  const startTime = Date.now();
  next();
  const endTime = Date.now();
  const ellapsedTime = (endTime - startTime) / 1000;
  summary
    .labels({
      error: 'none',
      exception: 'none',
      method: req.method.toUpperCase(),
      uri: req.path,
      outcome: 'SUCCESS',
      status: res.statusCode.toString(),
    })
    .observe(ellapsedTime);
});
app.get('/', (req, res, next) => {
  res.status(200).json({ message: 'OK', data: null });
  return;
});

app.get('/fibonacci', (req, res, next) => {
  const n = req.query['n'];
  if (n === undefined || n === null || n === '') {
    res.status(400).json({ message: 'n parameter required', data: null });
    return;
  }
  console.log(typeof n);
  if (typeof n !== 'string') {
    res.status(400).json({ message: 'n must be string', data: null });
    return;
  }

  const x = Number.parseInt(n as unknown as string);

  const data = fibonacci(x);
  res.status(200).json({ message: 'OK', data: data });
  return;
});
app.get('/metrics', async (req, res, next) => {
  res.setHeader('Content-Type', register.contentType);
  res.status(200).send(await register.metrics());
  return;
});

app.listen(port, () => {
  console.log(`Server running on port ${port}`);
});

function fibonacci(n: number): number {
  if (n <= 1) {
    return n;
  }

  return fibonacci(n - 1) + fibonacci(n - 2);
}
