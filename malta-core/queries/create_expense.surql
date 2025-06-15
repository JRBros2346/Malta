CREATE expense:[
	time::group($on_date, 'day'),
	$project
] SET reason = $reason, amount = $amount;