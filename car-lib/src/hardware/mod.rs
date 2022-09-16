pub mod servo;

const MIN_PULSE_WIDTH: usize = 544; // the shortest pulse sent to a servo
const MAX_PULSE_WIDTH: usize = 2400; // the longest pulse sent to a servo
const DEFAULT_PULSE_WIDTH: usize = 1500; // default pulse width when servo is attached
const REFRESH_INTERVAL: usize = 20000; // minumim time to refresh servos in microseconds

const SERVOS_PER_TIMER: usize = 12; // the maximum number of servos controlled by one timer
