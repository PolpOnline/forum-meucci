import { StatusCodes } from 'http-status-codes';

export async function GET() {
	return new Response('OK', { status: StatusCodes.OK });
}
