#![allow(dead_code)]

use main;

pub fn interrupt() {
	loop {}
}

type IVFunction = fn ();

struct IVTable {
	main: IVFunction,
	nmi_handler: IVFunction,
	hard_fault: IVFunction,
	bus_fault: IVFunction,
	usage_fault: IVFunction,
}

#[link_section = ".ivtable"]
#[used]
static IVTABLE: IVTable = IVTable { 
	main: main,
	nmi_handler: interrupt,
	hard_fault: interrupt,
	bus_fault: interrupt,
	usage_fault: interrupt
};

