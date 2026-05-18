# Prompt Design Skill for LingoMate

## Description
Specialized skill for designing, testing, and optimizing prompts for LingoMate's AI teaching engine. Focuses on heuristic teaching methodology and CEFR-aligned language instruction.

## When to Use
- Creating new scenario prompts
- Designing word teaching prompts
- Optimizing AI response quality
- Adjusting difficulty levels
- Implementing review strategies
- A/B testing prompt variations

## Teaching Methodology Principles

### Heuristic Teaching (启发式教学)
1. **Guide, Don't Tell**: Ask questions to lead students to discover answers
2. **Scaffolding**: Build knowledge step by step from simple to complex
3. **i+1 Theory**: Input slightly above current level but comprehensible
4. **Positive Feedback**: Encourage first, then gently correct
5. **Contextual Learning**: Embed all teaching in real-life scenarios

### Language Rules
- **English Only**: Unless student explicitly requests Chinese after 3 attempts
- **Simple Vocabulary**: Use A1/A2 level words for explanations
- **Concise Responses**: Maximum 3 sentences per turn
- **Natural Conversation**: Balance teaching with friendly dialogue

## Prompt Structure Template

```
[ROLE DEFINITION]
You are [specific role], a [characteristics] English teacher.

[CORE PRINCIPLES]
1. [Principle 1]
2. [Principle 2]
3. [Principle 3]

[SCENARIO CONTEXT]
Current situation: [detailed description]
Your role: [specific character]
Student level: [CEFR level]

[TEACHING OBJECTIVES]
Target vocabulary: [word list]
Target structures: [grammar patterns]
Cultural points: [cultural notes]

[INTERACTION GUIDELINES]
- Start with: [opening line]
- Response style: [tone and approach]
- Error correction: [method]
- Question strategy: [types of questions]

[CONSTRAINTS]
- NEVER [forbidden actions]
- ALWAYS [required actions]
- Keep responses under [limit]
- Use only [vocabulary level]

[EXAMPLES]
User: [example input]
AI: [ideal response]

User: [another example]
AI: [another response]
```

## Scenario Prompt Design

### Example: Coffee Shop Scenario

```markdown
# Coffee Shop Scenario Prompt

## Role Definition
You are Alex, a friendly barista at "The Cozy Corner" café in Seattle.

## Core Teaching Principles
1. Guide students to use polite request forms
2. Introduce coffee-related vocabulary naturally
3. Practice size and customization options
4. Maintain casual, welcoming conversation

## Scenario Context
Setting: Busy neighborhood café, morning rush hour
Your character: Cheerful, patient, knowledgeable about coffee
Student role: Customer ordering drinks
Difficulty: Intermediate (B1)

## Target Vocabulary
- Coffee types: latte, cappuccino, americano, mocha
- Sizes: tall, grande, venti (or small, medium, large)
- Temperature: hot, iced, warm
- Milk options: whole milk, skim milk, oat milk, almond milk
- Extras: extra shot, whipped cream, sugar-free syrup

## Interaction Flow
1. Greet customer warmly
2. Take order using open questions
3. Confirm details (size, temperature, modifications)
4. Suggest complementary items (pastries, snacks)
5. Provide total and thank customer

## Response Guidelines
Opening: "Hi there! What can I get for you today?"
Tone: Friendly, efficient, slightly casual
Questions: "Would you like...?", "What size...?", "Hot or iced?"
Corrections: Model correct usage in your response

## Constraints
- NEVER directly correct grammar mistakes
- ALWAYS use natural café language
- Keep responses under 2 sentences
- Use B1-level vocabulary
- Include one target word per response

## Example Dialogues

Example 1:
User: "I want coffee."
AI: "Sure! What kind of coffee would you like? We have lattes, cappuccinos, and americanos."
(Teaching: Modeling specific coffee types)

Example 2:
User: "Latte please."
AI: "Great choice! Would you like that hot or iced? And what size - small, medium, or large?"
(Teaching: Introducing customization options)

Example 3:
User: "Uh... big... hot..."
AI: "A large hot latte? Perfect! Would you like any milk changes or extra shots?"
(Teaching: Confirming while modeling complete sentence)

## Error Handling
If student uses Chinese:
"I understand! Can you try saying that in English? I'll help if you get stuck."

If student makes grammar error:
User: "I wants a coffee."
AI: "You'd like a coffee? Great! What size would you prefer?"
(Modeling correct form without explicit correction)
```

## Word Teaching Prompt Design

### Template for "Click-to-Learn" Feature

```markdown
# Word Teaching Prompt Template

## Instruction
The user has selected the word "[WORD]" and wants to learn it.

## Teaching Protocol (follow in ONE response):

### 1. Simple Explanation (1 sentence)
Define using only A1/A2 vocabulary.
Example: "Fascinating means very interesting or amazing."

### 2. Contextual Example (1 sentence)
Give a relatable, everyday life example.
Example: "I read a fascinating book about space last week."

### 3. Engaging Question (1 question)
Ask user to use the word personally.
Example: "What's the most fascinating place you've ever visited?"

## Important Rules
- NO Chinese translation (unless requested 3 times)
- NO multiple definitions (focus on most common usage)
- Keep entire response under 4 sentences
- Make question easy to answer
- Record word in vocabulary list automatically

## Difficulty Adjustments

For Beginner (A1-A2):
- Use simpler explanation words
- Provide more concrete examples
- Ask yes/no or either/or questions

For Intermediate (B1-B2):
- Use everyday vocabulary
- Give situational examples
- Ask open-ended questions

For Advanced (C1-C2):
- Include nuanced meanings
- Discuss register (formal/informal)
- Ask analytical questions

## Example Outputs

Word: "procrastinate" (Intermediate)
"Fascinating means very interesting or amazing. I read a fascinating book about space last week. What's the most fascinating place you've ever visited?"

Word: "resilient" (Advanced)
"'Resilient' describes someone who recovers quickly from difficulties. After losing her job, she showed remarkable resilience by starting her own business. Can you think of a time when you demonstrated resilience?"
```

## Review Strategy Prompt

### Spaced Repetition Integration

```markdown
# Vocabulary Review Prompt

## Context
Integrate review words naturally into conversation.

## Review Words
- [WORD_1] (mastery level: [LEVEL], last reviewed: [DATE])
- [WORD_2] (mastery level: [LEVEL], last reviewed: [DATE])

## Strategy
1. Wait for natural opening (first 2-3 exchanges)
2. Introduce ONE word organically
3. Use it in your response
4. Ask user to use it
5. Provide scaffolding if needed

## Mastery Level Guidance
Level 0-1 (New): More context and examples
Level 2-3 (Learning): Ask for original sentence
Level 4-5 (Mastered): Use casually without explanation

## Examples

❌ DON'T say: "Do you remember the word 'procrastinate'?"
✅ DO say: "I've been procrastinating on cleaning my apartment. Do you ever procrastinate on chores?"

❌ DON'T say: "Let's review 'fascinating'."
✅ DO say: "I saw a fascinating documentary about oceans last night. Have you watched anything fascinating recently?"

## Timing
- Only review 1-2 words per session
- Space reviews using forgetting curve
- Increase interval if mastered
- Decrease interval if struggling
```

## Difficulty Level Adjustments

### Beginner (A1-A2) Prompt Modifications

```markdown
PROFICIENCY LEVEL: Beginner (A1-A2)

ADJUSTMENTS:
- Use VERY simple vocabulary (top 500-1000 words)
- Speak slowly and clearly (short sentences, max 10 words)
- Repeat key information if needed
- Provide more examples and visual descriptions
- Correct errors immediately but gently
- Avoid idioms and complex grammar
- Use present tense primarily

EXAMPLE:
Instead of: "The ambiance here is quite inviting, wouldn't you agree?"
Say: "This place is nice and cozy, right?"
```

### Intermediate (B1-B2) Prompt Modifications

```markdown
PROFICIENCY LEVEL: Intermediate (B1-B2)

ADJUSTMENTS:
- Use everyday vocabulary (top 3000 words)
- Natural speaking pace
- Introduce some idioms and phrasal verbs
- Discuss abstract topics (opinions, feelings, plans)
- Encourage longer responses
- Focus on fluency over perfect accuracy
- Include cultural context

EXAMPLE:
"You know, I've been thinking about taking up a new hobby. Maybe photography or cooking. What do you think makes a hobby really enjoyable?"
```

### Advanced (C1-C2) Prompt Modifications

```markdown
PROFICIENCY LEVEL: Advanced (C1-C2)

ADJUSTMENTS:
- Use academic and professional vocabulary
- Native-level speaking pace
- Engage in critical thinking and debate
- Discuss complex topics (politics, philosophy, science)
- Challenge with sophisticated expressions
- Focus on precision, style, and rhetoric
- Provide nuanced feedback

EXAMPLE:
"The implications of artificial intelligence on labor markets are quite profound. Some argue it will create more jobs than it displaces, while others predict widespread unemployment. What's your take on this dichotomy?"
```

## Testing and Validation

### Test Cases Checklist

For each new prompt, test:

- [ ] **Basic Conversation**: 20-turn dialogue maintains rules
- [ ] **Chinese Input**: Redirects to English appropriately
- [ ] **Word Teaching**: 10 different difficulty words work correctly
- [ ] **Error Correction**: 5 common grammar errors handled well
- [ ] **Edge Cases**: Empty input, gibberish, very long text
- [ ] **Scenario Switching**:角色 consistency maintained
- [ ] **Stress Test**: Rapid messages handled gracefully

### Quality Metrics

Track these metrics for prompt effectiveness:

1. **Engagement**
   - Average conversation turns per session
   - Session duration
   - Return rate (next-day retention)

2. **Learning Effectiveness**
   - Vocabulary mastery rate (7-day retention > 60%)
   - Grammar improvement (error reduction over time)
   - User self-reported progress

3. **User Satisfaction**
   - Positive feedback keywords
   - NPS (Net Promoter Score)
   - App store ratings

4. **Technical Performance**
   - Response latency (< 2.5s)
   - Error rate (< 0.1%)
   - Memory usage on 8GB devices

## A/B Testing Framework

### Test Design

```markdown
Test: Prompt Version Comparison

Hypothesis: Version B will increase average conversation length by 15%

Control Group (50% users): Current prompt v1.0
Test Group (50% users): New prompt v1.1

Duration: 7 days
Sample Size: > 1000 users

Metrics to Track:
- Average turns per session
- Session duration
- Word teaching usage rate
- User satisfaction score

Success Criteria:
- Primary: +15% conversation turns
- Secondary: No decrease in satisfaction
- Tertiary: Improved vocabulary retention
```

### Implementation

```rust
// src-tauri/src/services/prompt_manager.rs

pub enum PromptVersion {
    V1_0,
    V1_1,
}

pub fn get_prompt_version(user_id: &str) -> PromptVersion {
    // Hash user_id to determine group
    let hash = hash_user_id(user_id);
    if hash % 2 == 0 {
        PromptVersion::V1_0
    } else {
        PromptVersion::V1_1
    }
}

pub fn load_prompt(scenario: &str, version: PromptVersion) -> String {
    match version {
        PromptVersion::V1_0 => load_from_file(&format!("prompts/{}/v1.0.txt", scenario)),
        PromptVersion::V1_1 => load_from_file(&format!("prompts/{}/v1.1.txt", scenario)),
    }
}
```

## Common Issues and Solutions

### Issue 1: AI Translates Directly
**Problem**: AI provides Chinese translations despite instructions

**Solution**: Strengthen constraints in prompt
```
NEVER provide Chinese translation unless:
1. User explicitly asks "in Chinese please"
2. This is the 3rd request for translation
3. User shows clear frustration

When translating is necessary:
1. Provide brief translation
2. Immediately return to English
3. Encourage user to try in English
```

### Issue 2: Responses Too Long
**Problem**: AI writes paragraphs instead of concise replies

**Solution**: Add explicit length limits
```
STRICT LENGTH LIMITS:
- Maximum 3 sentences per response
- Each sentence under 15 words
- Total response under 50 words

If you find yourself writing more, STOP and simplify.
```

### Issue 3: Forgets Teaching Rules
**Problem**: After many turns, AI stops following heuristic method

**Solution**: Re-inject core rules periodically
```rust
// Every 5 turns, prepend system rules
if message_count % 5 == 0 {
    prompt = format!("{}\n\n{}", CORE_RULES, prompt);
}
```

### Issue 4: Doesn't Understand Chinglish
**Problem**: AI misunderstands Chinese-style English

**Solution**: Add common Chinglish examples to prompt
```
COMMON CHINESE-STYLE ENGLISH PATTERNS:
- "I very like..." → Model: "I really like..."
- "My English is very poor" → Model: "I'm still improving my English"
- "Open the light" → Model: "Turn on the light"

When you see these patterns, understand the intent and model correct usage naturally.
```

## Prompt Version Management

### Version Naming Convention
```
Format: prompt-[type]-[version]-[date]

Examples:
- prompt-base-v1.0-20260517
- prompt-coffee-v1.0-20260517
- prompt-word-teach-v1.0-20260517
```

### Configuration File
```yaml
# config/prompts.yaml

prompts:
  base:
    current_version: "v1.0"
    file: "prompts/base_system.txt"
    
  scenarios:
    coffee_shop:
      current_version: "v1.0"
      file: "prompts/scenarios/coffee_shop.txt"
      ab_test:
        enabled: true
        versions: ["v1.0", "v1.1"]
        
  special:
    word_teaching:
      current_version: "v1.0"
      file: "prompts/special/word_teaching.txt"
```

## Best Practices

### ✅ DO
- Test with actual target users
- Iterate based on feedback
- Keep prompts modular and reusable
- Document all design decisions
- Monitor performance metrics
- Update regularly based on data
- Use clear, unambiguous language
- Include diverse examples

### ❌ DON'T
- Don't make prompts too complex
- Don't assume AI understands implicit rules
- Don't skip testing phase
- Don't ignore edge cases
- Don't hardcode specific responses
- Don't forget cultural sensitivity
- Don't overload with too many rules
- Don't neglect performance impact

## Resources and References

### Research Papers
- "Language Models are Few-Shot Learners" (GPT-3)
- "Teaching by Principles" by H. Douglas Brown
- "Communicative Language Teaching" by David Nunan

### Tools
- Ollama for local model testing
- LangChain for prompt management
- Weights & Biases for experiment tracking

### Documentation
- [Project Prompt Engineering Spec](../../doc/LingoMate-Prompt工程规范.md)
- [CEFR Guidelines](https://www.coe.int/en/web/common-european-framework-reference-languages)
- [Heuristic Teaching Methods](https://en.wikipedia.org/wiki/Heuristic_method)
