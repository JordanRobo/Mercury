<script lang="ts">
	import * as Card from "$lib/components/ui/card/index.js";
    import * as Table from "$lib/components/ui/table";
    import { Badge } from "$lib/components/ui/badge/index.js";
    import { Button } from "$lib/components/ui/button/index.js";

	import { ArrowTopRight, BarChart, Person } from "svelte-radix";
</script>

<div class="grid grid-cols-2 gap-4 p-12">
    <Card.Root>
        <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2" >
            <Card.Title class="text-sm font-medium">Page Views</Card.Title>
            <BarChart class="h-4 w-4 text-muted-foreground" />
        </Card.Header>
        <Card.Content>
            <div class="text-2xl font-bold">69,420 <span class="text-muted-foreground text-sm">In last 30 days</span></div>
            <p class="text-xs text-muted-foreground">+20.1% from last period</p>
        </Card.Content>
    </Card.Root>
    <Card.Root>
        <Card.Header
            class="flex flex-row items-center justify-between space-y-0 pb-2"
        >
            <Card.Title class="text-sm font-medium">Subscriptions</Card.Title>
            <Person class="h-4 w-4 text-muted-foreground" />
        </Card.Header>
        <Card.Content>
            <div class="text-2xl font-bold">+2350</div>
            <p class="text-xs text-muted-foreground">+180.1% from last month</p>
        </Card.Content>
    </Card.Root>
    <div class="col-span-2">
        <Card.Root class="xl:col-span-2">
            <Card.Header class="flex flex-row items-center">
                <div class="grid gap-2">
                    <Card.Title>Transactions</Card.Title>
                    <Card.Description>Recent transactions from your store.</Card.Description>
                </div>
                <Button href="##" size="sm" class="ml-auto gap-1">
                    View All
                    <ArrowTopRight class="h-4 w-4" />
                </Button>
            </Card.Header>
            <Card.Content>
                <Table.Root>
                    <Table.Header>
                        <Table.Row>
                            <Table.Head>Customer</Table.Head>
                            <Table.Head class="xl:table.-column hidden">Type</Table.Head>
                            <Table.Head class="xl:table.-column hidden">Status</Table.Head>
                            <Table.Head class="xl:table.-column hidden">Date</Table.Head>
                            <Table.Head class="text-right">Amount</Table.Head>
                        </Table.Row>
                    </Table.Header>
                    <Table.Body>
                        <Table.Row>
                            <Table.Cell>
                                <div class="font-medium">Liam Johnson</div>
                                <div class="hidden text-sm text-muted-foreground md:inline">
                                    liam@example.com
                                </div>
                            </Table.Cell>
                            <Table.Cell class="xl:table.-column hidden">Sale</Table.Cell>
                            <Table.Cell class="xl:table.-column hidden">
                                <Badge class="text-xs" variant="outline">Approved</Badge>
                            </Table.Cell>
                            <Table.Cell class="md:table.-cell xl:table.-column hidden lg:hidden">
                                2023-06-23
                            </Table.Cell>
                            <Table.Cell class="text-right">$250.00</Table.Cell>
                        </Table.Row>
                        <Table.Row>
                            <Table.Cell>
                                <div class="font-medium">Olivia Smith</div>
                                <div class="hidden text-sm text-muted-foreground md:inline">
                                    olivia@example.com
                                </div>
                            </Table.Cell>
                            <Table.Cell class="xl:table.-column hidden">Refund</Table.Cell>
                            <Table.Cell class="xl:table.-column hidden">
                                <Badge class="text-xs" variant="outline">Declined</Badge>
                            </Table.Cell>
                            <Table.Cell class="md:table.-cell xl:table.-column hidden lg:hidden">
                                2023-06-24
                            </Table.Cell>
                            <Table.Cell class="text-right">$150.00</Table.Cell>
                        </Table.Row>
                        <Table.Row>
                            <Table.Cell>
                                <div class="font-medium">Noah Williams</div>
                                <div class="hidden text-sm text-muted-foreground md:inline">
                                    noah@example.com
                                </div>
                            </Table.Cell>
                            <Table.Cell class="xl:table.-column hidden">Subscription</Table.Cell>
                            <Table.Cell class="xl:table.-column hidden">
                                <Badge class="text-xs" variant="outline">Approved</Badge>
                            </Table.Cell>
                            <Table.Cell class="md:table.-cell xl:table.-column hidden lg:hidden">
                                2023-06-25
                            </Table.Cell>
                            <Table.Cell class="text-right">$350.00</Table.Cell>
                        </Table.Row>
                        <Table.Row>
                            <Table.Cell>
                                <div class="font-medium">Emma Brown</div>
                                <div class="hidden text-sm text-muted-foreground md:inline">
                                    emma@example.com
                                </div>
                            </Table.Cell>
                            <Table.Cell class="xl:table.-column hidden">Sale</Table.Cell>
                            <Table.Cell class="xl:table.-column hidden">
                                <Badge class="text-xs" variant="outline">Approved</Badge>
                            </Table.Cell>
                            <Table.Cell class="md:table.-cell xl:table.-column hidden lg:hidden">
                                2023-06-26
                            </Table.Cell>
                            <Table.Cell class="text-right">$450.00</Table.Cell>
                        </Table.Row>
                        <Table.Row>
                            <Table.Cell>
                                <div class="font-medium">Liam Johnson</div>
                                <div class="hidden text-sm text-muted-foreground md:inline">
                                    liam@example.com
                                </div>
                            </Table.Cell>
                            <Table.Cell class="xl:table.-column hidden">Sale</Table.Cell>
                            <Table.Cell class="xl:table.-column hidden">
                                <Badge class="text-xs" variant="outline">Approved</Badge>
                            </Table.Cell>
                            <Table.Cell class="md:table.-cell xl:table.-column hidden lg:hidden">
                                2023-06-27
                            </Table.Cell>
                            <Table.Cell class="text-right">$550.00</Table.Cell>
                        </Table.Row>
                    </Table.Body>
                </Table.Root>
            </Card.Content>
        </Card.Root>
    </div>
</div>