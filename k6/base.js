export const baseOptions = {
  stages: [
    { duration: "30s", target: 20 },
    { duration: "1m30s", target: 30 },
    { duration: "1m30s", target: 40 },
    { duration: "3m", target: 100 },
    { duration: "1m30s", target: 30 },
    { duration: "1m", target: 10 },
    { duration: "30s", target: 0 },
  ],
};

export const generateRandomNumber = () => {
  return Math.floor(Math.random() * 10) + 1;
};
