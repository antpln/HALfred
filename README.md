# HALfred
Hardware Abstraction Layer for ATMega 328 written in Rust


[CORRECTION GPIO] (Don't hesitate to remove this part)
It would be nice to have something to prevent modifying the register in an incoherent way. For example, if I do ``` let gpio = Gpio::new(50, Atmega_Port::PortB).```, it won't bug during the compilation, but it may generate a problem on your hardware.