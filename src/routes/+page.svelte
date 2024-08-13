<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import * as Card from "$lib/components/ui/card";
  import { Progress } from "$lib/components/ui/progress";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Assignment } from "$lib/types";
  import * as Table from "$lib/components/ui/table";
  import { Badge } from "$lib/components/ui/badge";
  import { onMount } from "svelte";

  let all_assignments: Assignment[] = [];
  let unsubmitted_assignments: Assignment[] = [];
  let error: any = null;
  let loading: boolean = true;
  let submittedPercentage: number = 0;
  let remainingAssignments: number = 0;

  async function loadAssignments() {
    try {
      loading = true;
      all_assignments = await invoke('get_all_assignments');
      unsubmitted_assignments = await invoke('get_unsubmitted_assignments');
      calculateStats();
    } catch (err) {
      error = err;
      console.error('Error loading assignments:', err);
    } finally {
      loading = false;
    }
  }

  function calculateStats() {
    const total = all_assignments.length;
    const unsubmitted = unsubmitted_assignments.length;
    const submitted = total - unsubmitted;
    
    submittedPercentage = total > 0 ? (submitted / total) * 100 : 0;
    remainingAssignments = unsubmitted;
  }

  async function submitAssignment(id: number) {
    try {
      await invoke('submit_assignment', { assId: id });
      // Refresh the assignments after submission
      await loadAssignments();
    } catch (err) {
      console.error('Error submitting assignment:', err);
      // Handle error (e.g., show an error message to the user)
    }
  }

  onMount(loadAssignments);

</script>

<main class="grid grid-cols-4 gap-4 p-4">
    <Card.Root class="col-span-2">
      <Card.Header class="pb-3">
        <Card.Title class="text-2xl">Certificate Progress Tracker</Card.Title>
        <Card.Description class="max-w-lg text-balance leading-relaxed">
          Introducing Our Dynamic Certificate Progress Tracker Dashboard for Seamless Management and
          Insightful Analysis of Course Progress.
        </Card.Description>
      </Card.Header>
    </Card.Root>
    <Card.Root>
      <Card.Header class="pb-2">
        <Card.Description>Submitted Assignments</Card.Description>
        <Card.Title class="text-4xl">{submittedPercentage.toFixed(1)}%</Card.Title>
      </Card.Header>
      <Card.Content>
        <Progress value={submittedPercentage} aria-label="{submittedPercentage}% submitted" />
      </Card.Content>
    </Card.Root>
    <Card.Root>
      <Card.Header class="pb-2">
        <Card.Description>Assignments Remaining</Card.Description>
        <Card.Title class="text-4xl">{remainingAssignments}</Card.Title>
      </Card.Header>
      <Card.Content>
        <Progress value={100 - submittedPercentage} aria-label="{remainingAssignments} assignments remaining" />
      </Card.Content>
    </Card.Root>
</main>
<div class="p-4">
  
  <Table.Root>
    <Table.Caption>Unsubmitted Assignments</Table.Caption>
    <Table.Header>
      <Table.Row>
        <Table.Head class="w-[100px]">ID</Table.Head>
        <Table.Head>Name</Table.Head>
        <Table.Head>Type</Table.Head>
        <Table.Head class="text-center">Action</Table.Head>
      </Table.Row>
    </Table.Header>
    <Table.Body>
      {#if loading}
        <Table.Row>
          <Table.Cell>Loading Assignments...</Table.Cell>
        </Table.Row>
      {:else if error}
        <Table.Row>
          <Table.Cell>Error: {error}</Table.Cell>
        </Table.Row>
      {:else if unsubmitted_assignments.length === 0}
        <Table.Row>
          <Table.Cell>No unsubmitted assignments</Table.Cell>
        </Table.Row>
      {:else}
        {#each unsubmitted_assignments as item}
          <Table.Row>
            <Table.Cell class="font-medium">{item.id}</Table.Cell>
            <Table.Cell>
              <div class="font-medium">{item.name}</div>
              <div class="text-muted-foreground hidden text-sm md:inline">{item.unit}</div>
            </Table.Cell>
            <Table.Cell><Badge variant="outline">{item.assessment_type}</Badge></Table.Cell>
            <Table.Cell class="text-center">
              <Button on:click={() => submitAssignment(item.id)}>Submit</Button>
            </Table.Cell>
          </Table.Row>
        {/each}
      {/if}
    </Table.Body>
  </Table.Root>
</div>
