// section: 	--types
export interface ChartDateObject {
	hour: number;
	date: number;
	day: string;

	month: number;
	year: number;
}

export const DaysOfTheWeek: string[] = [
	'Sunday',
	'Monday',
	'Tuesday',
	'Wednesday',
	'Thursday',
	'Friday',
	'Saturday'
];
// endsection:  --types
export function convertIsoToDate(isoDateString: string): string {
	const date = new Date(isoDateString);

	return `${date.getDate()} - ${date.getMonth()} - ${date.getFullYear()}`;
}

export function getChartDatesObject(isoDateString: string): ChartDateObject {
	const date = new Date(isoDateString);

	const chartDateObject: ChartDateObject = {
		hour: date.getHours(),
		date: date.getDate(),
		day: DaysOfTheWeek[date.getDay()],
		month: date.getMonth(),
		year: date.getFullYear()
	};

	return chartDateObject;
}

export function generateDates(): string[] {
	const dates: string[] = [];

	for (let i = 0; i <= 31; i++) {
		dates.push(`${i}`);
	}

	return dates;
}