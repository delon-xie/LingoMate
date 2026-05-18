
import type { Scenario } from '../../types';

interface ScenarioCardProps {
  scenario: Scenario;
  onClick: () => void;
}

const scenarioConfig: Record<Scenario, { icon: string; title: string; description: string; category?: string }> = {
  // 日常生活 (Daily Life)
  coffee_shop: {
    icon: '☕',
    title: 'Coffee Shop',
    description: 'Order drinks and chat with baristas',
    category: 'Daily Life',
  },
  restaurant: {
    icon: '🍽️',
    title: 'Restaurant',
    description: 'Make reservations and order food',
    category: 'Daily Life',
  },
  grocery_shopping: {
    icon: '🛒',
    title: 'Grocery Shopping',
    description: 'Buy groceries and ask about products',
    category: 'Daily Life',
  },
  pharmacy: {
    icon: '💊',
    title: 'Pharmacy',
    description: 'Get medicine and health advice',
    category: 'Daily Life',
  },
  banking: {
    icon: '🏦',
    title: 'Banking',
    description: 'Open accounts and manage finances',
    category: 'Daily Life',
  },
  post_office: {
    icon: '📮',
    title: 'Post Office',
    description: 'Send packages and mail letters',
    category: 'Daily Life',
  },
  
  // 旅行交通 (Travel & Transportation)
  hotel_checkin: {
    icon: '🏨',
    title: 'Hotel Check-in',
    description: 'Check in and ask about amenities',
    category: 'Travel',
  },
  airport: {
    icon: '✈️',
    title: 'Airport',
    description: 'Navigate airports and check in flights',
    category: 'Travel',
  },
  taxi_rideshare: {
    icon: '🚕',
    title: 'Taxi & Rideshare',
    description: 'Book rides and give directions',
    category: 'Travel',
  },
  train_station: {
    icon: '🚂',
    title: 'Train Station',
    description: 'Buy tickets and ask about schedules',
    category: 'Travel',
  },
  car_rental: {
    icon: '🚗',
    title: 'Car Rental',
    description: 'Rent a car and understand policies',
    category: 'Travel',
  },
  directions_asking: {
    icon: '🗺️',
    title: 'Asking Directions',
    description: 'Find your way around the city',
    category: 'Travel',
  },
  
  // 工作商务 (Work & Business)
  job_interview: {
    icon: '💼',
    title: 'Job Interview',
    description: 'Practice common interview questions',
    category: 'Work',
  },
  office_meeting: {
    icon: '👥',
    title: 'Office Meeting',
    description: 'Participate in team discussions',
    category: 'Work',
  },
  business_call: {
    icon: '📞',
    title: 'Business Call',
    description: 'Handle professional phone conversations',
    category: 'Work',
  },
  email_writing: {
    icon: '📧',
    title: 'Email Writing',
    description: 'Write professional emails',
    category: 'Work',
  },
  presentation: {
    icon: '📊',
    title: 'Presentation',
    description: 'Present ideas and answer questions',
    category: 'Work',
  },
  networking_event: {
    icon: '🤝',
    title: 'Networking Event',
    description: 'Introduce yourself professionally',
    category: 'Work',
  },
  
  // 社交娱乐 (Social & Entertainment)
  social_gathering: {
    icon: '🎉',
    title: 'Social Gathering',
    description: 'Meet new people and make friends',
    category: 'Social',
  },
  dating: {
    icon: '💕',
    title: 'Dating',
    description: 'Practice casual conversation on dates',
    category: 'Social',
  },
  movie_theater: {
    icon: '🎬',
    title: 'Movie Theater',
    description: 'Buy tickets and discuss movies',
    category: 'Social',
  },
  gym_fitness: {
    icon: '💪',
    title: 'Gym & Fitness',
    description: 'Ask about equipment and classes',
    category: 'Social',
  },
  doctor_appointment: {
    icon: '🏥',
    title: 'Doctor Appointment',
    description: 'Describe symptoms and get advice',
    category: 'Social',
  },
  shopping_clothes: {
    icon: '👕',
    title: 'Clothing Store',
    description: 'Shop for clothes and ask for sizes',
    category: 'Social',
  },
};

export const ScenarioCard: React.FC<ScenarioCardProps> = ({
  scenario,
  onClick,
}) => {
  const config = scenarioConfig[scenario];

  const handleClick = (e: React.MouseEvent) => {
    console.log('ScenarioCard onClick triggered for:', scenario);
    console.log('Event:', e.type);
    onClick();
  };

  return (
    <div
      onClick={handleClick}
      className="bg-gray-50 border-2 border-transparent rounded-xl p-6 cursor-pointer transition-all duration-200 hover:border-primary-500 hover:bg-primary-50 hover:-translate-y-0.5"
    >
      <div className="text-5xl mb-3">{config.icon}</div>
      <h3 className="text-lg font-semibold text-gray-900 mb-1">{config.title}</h3>
      <p className="text-sm text-gray-500">{config.description}</p>
    </div>
  );
};
