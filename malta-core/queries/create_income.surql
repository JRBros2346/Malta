CREATE income:[
	time::group($on_date, 'day'),
	$source
] SET amount = $amount;