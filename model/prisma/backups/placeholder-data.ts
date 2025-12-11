// This file contains placeholder data that you'll be replacing with real data in the Data Fetching chapter:
// https://nextjs.org/learn/dashboard-app/fetching-data
const users = [
  {
    id: '410544b2-4001-4271-9855-fec4b6a6442a',
    name: 'User',
    email: 'user@nextmail.com',
    password: '123456',
  },
];

const customers = [
  {
    id: '',
    name: 'Evil Rabbit',
    email: 'evil@rabbit.com',
    imageUrl: '/customers/evil-rabbit.png',
  },
  {
    id: '',
    name: 'Delba de Oliveira',
    email: 'delba@oliveira.com',
    imageUrl: '/customers/delba-de-oliveira.png',
  },
  {
    id: '',
    name: 'Lee Robinson',
    email: 'lee@robinson.com',
    imageUrl: '/customers/lee-robinson.png',
  },
  {
    id: '',
    name: 'Michael Novotny',
    email: 'michael@novotny.com',
    imageUrl: '/customers/michael-novotny.png',
  },
  {
    id: '',
    name: 'Amy Burns',
    email: 'amy@burns.com',
    imageUrl: '/customers/amy-burns.png',
  },
  {
    id: '',
    name: 'Balazs Orban',
    email: 'balazs@orban.com',
    imageUrl: '/customers/balazs-orban.png',
  },
];

const invoices = [
  {
    customerId: 0,
    amount: 15795,
    status: 'PENDING',
    date: new Date('2022-12-06'),
  },
  {
    customerId: 1,
    amount: 20348,
    status: 'PENDING',
    date: new Date('2022-11-14'),
  },
  {
    customerId: 4,
    amount: 3040,
    status: 'PAID',
    date: new Date('2022-10-29'),
  },
  {
    customerId: 3,
    amount: 44800,
    status: 'PAID',
    date: new Date('2023-09-10'),
  },
  {
    customerId: 5,
    amount: 34577,
    status: 'PENDING',
    date: new Date('2023-08-05'),
  },
  {
    customerId: 2,
    amount: 54246,
    status: 'PENDING',
    date: new Date('2023-07-16'),
  },
  {
    customerId: 0,
    amount: 666,
    status: 'PENDING',
    date: new Date('2023-06-27'),
  },
  {
    customerId: 3,
    amount: 32545,
    status: 'PAID',
    date: new Date('2023-06-09'),
  },
  {
    customerId: 4,
    amount: 1250,
    status: 'PAID',
    date: new Date('2023-06-17'),
  },
  {
    customerId: 5,
    amount: 8546,
    status: 'PAID',
    date: new Date('2023-06-07'),
  },
  {
    customerId: 1,
    amount: 500,
    status: 'PAID',
    date: new Date('2023-08-19'),
  },
  {
    customerId: 5,
    amount: 8945,
    status: 'PAID',
    date: new Date('2023-06-03'),
  },
  {
    customerId: 2,
    amount: 1000,
    status: 'PAID',
    date: new Date('2022-06-05'),
  },
];

const revenue = [
  { month: 'Jan', revenue: 2000 },
  { month: 'Feb', revenue: 1800 },
  { month: 'Mar', revenue: 2200 },
  { month: 'Apr', revenue: 2500 },
  { month: 'May', revenue: 2300 },
  { month: 'Jun', revenue: 3200 },
  { month: 'Jul', revenue: 3500 },
  { month: 'Aug', revenue: 3700 },
  { month: 'Sep', revenue: 2500 },
  { month: 'Oct', revenue: 2800 },
  { month: 'Nov', revenue: 3000 },
  { month: 'Dec', revenue: 4800 },
];

export { users, customers, invoices, revenue };
