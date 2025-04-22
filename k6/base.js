export const baseOptions = {
  stages: [
    { duration: "30s", target: 20 },
    { duration: "1m", target: 30 },
    { duration: "30s", target: 0 },
  ],
};

export const generateRandomNumber = () => {
  return Math.floor(Math.random() * 10) + 1;
};
